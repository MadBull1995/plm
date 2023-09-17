use std::fmt;
use serde::{Serializer, ser::{SerializeMap, SerializeStruct}, de::{self, MapAccess, Visitor}, Deserializer, Deserialize};
use serde_json::Value;

/// Core protobuf schema of ppm
pub mod ppm {
    pub mod package {
        pub mod v1 {
            tonic::include_proto!("ppm.package.v1");
        }
    }
    pub mod registry {
        pub mod v1 {
            tonic::include_proto!("ppm.registry.v1");
        }
    }
}

/// A collection of helpers that shared to multiple logics
pub mod utils {
    pub mod fs;
}

// Custom serialization
impl serde::Serialize for ppm::package::v1::Manifest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(9))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("srcDir", &self.src_dir)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("license", &self.license)?;
        map.serialize_entry("authors", &self.authors)?;
        map.serialize_entry("repositoryUrl", &self.repository_url)?;
        map.serialize_entry("metadata", &self.metadata)?;
        map.serialize_entry("dependencies", &self.dependencies)?;
        map.end()
    }
}

impl serde::Serialize for ppm::registry::v1::S3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("S3", 1)?;
        s.serialize_field("bucketName", &self.bucket_name)?;
        s.end()
    }
}

impl serde::Serialize for ppm::registry::v1::Server {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Server", 1)?;
        s.serialize_field("port", &self.port)?;
        s.serialize_field("host", &self.host)?;
        s.end()
    }
}


impl serde::Serialize for ppm::registry::v1::Local {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Local", 1)?;
        s.serialize_field("registryPath", &self.registry_path)?;
        s.end()
    }
}


impl serde::Serialize for ppm::registry::v1::Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Data", 3)?;
        s.serialize_field("host", &self.host)?;
        s.serialize_field("username", &self.username)?;
        s.serialize_field("password", &self.password)?;
        s.end()
    }
}

impl serde::Serialize for ppm::registry::v1::storage::StorageBackend {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut map = serializer.serialize_map(Some(1))?;

        match self {
            ppm::registry::v1::storage::StorageBackend::Local(ref local) => {
                map.serialize_entry("local", local)?;
            },
            ppm::registry::v1::storage::StorageBackend::S3(ref s3) => {
                map.serialize_entry("s3", s3)?;
            },
        }

        map.end()
    }
}


impl<'de> serde::Deserialize<'de> for ppm::registry::v1::Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ConfigVisitor;

        impl<'de> serde::de::Visitor<'de> for ConfigVisitor {
            type Value = ppm::registry::v1::Config;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Config")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut storage: Option<ppm::registry::v1::Storage> = None;
                let mut server: Option<ppm::registry::v1::Server> = None;
                let mut data: Option<ppm::registry::v1::Data> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "storage" => {
                            if storage.is_some() {
                                return Err(serde::de::Error::duplicate_field("storage"));
                            }
                            storage = Some(map.next_value()?);
                        },
                        "server" => {
                            if server.is_some() {
                                return Err(serde::de::Error::duplicate_field("server"));
                            }
                            server = Some(map.next_value()?);
                        },
                        "data" => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        },
                        _ => {
                            // Unknown field, you can decide how to handle this
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        },
                    }
                }

                Ok(ppm::registry::v1::Config {
                    storage,
                    server,
                    data,
                })
            }
        }

        deserializer.deserialize_struct("Config", &["storage", "server", "data"], ConfigVisitor)
    }
}


impl<'de> Deserialize<'de> for Storage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let backend = StorageBackend::deserialize(deserializer)?;
        Ok(ppm::registry::v1::Storage { storage_backend: Some(backend) })
    }
}

impl<'de> Deserialize<'de> for StorageBackend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StorageVisitor;

        impl<'de> Visitor<'de> for StorageVisitor {
            type Value = StorageBackend;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("one of storage_backend: ['local', 's3']")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut local: Option<Local> = None;
                let mut s3: Option<S3> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "local" => {
                            if local.is_some() {
                                return Err(de::Error::duplicate_field("local"));
                            }
                            local = Some(map.next_value()?);
                        }
                        "s3" => {
                            if s3.is_some() {
                                return Err(de::Error::duplicate_field("s3"));
                            }
                            s3 = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(&key, &["local", "s3"])),
                    }
                }

                if let Some(local) = local {
                    Ok(StorageBackend::Local(local))
                } else if let Some(s3) = s3 {
                    Ok(StorageBackend::S3(s3))
                } else {
                    Err(de::Error::missing_field("storage_backend"))
                }
            }
        }

        deserializer.deserialize_map(StorageVisitor)
    }
}


impl<'de> Deserialize<'de> for S3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct S3Visitor;

        impl<'de> Visitor<'de> for S3Visitor {
            type Value = S3;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct S3 with field bucket_name")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut bucket_name = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "bucketName" {
                        bucket_name = Some(map.next_value()?);
                    } else {
                        return Err(de::Error::unknown_field(&key, &["bucketName"]));
                    }
                }

                let bucket_name = bucket_name.ok_or_else(|| de::Error::missing_field("bucketName"))?;
                Ok(S3 { bucket_name })
            }
        }

        deserializer.deserialize_map(S3Visitor)
    }
}


impl<'de> Deserialize<'de> for Local {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocalVisitor;

        impl<'de> Visitor<'de> for LocalVisitor {
            type Value = Local;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Local with field registry_path")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut registry_path = None;

                while let Some(key) = map.next_key::<String>()? {
                    if key == "registryPath" {
                        registry_path = Some(map.next_value()?);
                    } else {
                        return Err(de::Error::unknown_field(&key, &["registryPath"]));
                    }
                }

                let registry_path = registry_path.ok_or_else(|| de::Error::missing_field("registryPath"))?;
                Ok(Local { registry_path })
            }
        }

        deserializer.deserialize_map(LocalVisitor)
    }
}


impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataVisitor;

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Data with fields host, username, and password")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut host = None;
                let mut username = None;
                let mut password = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "host" => {
                            if host.is_some() {
                                return Err(de::Error::duplicate_field("host"));
                            }
                            host = Some(map.next_value()?);
                        },
                        "username" => {
                            if username.is_some() {
                                return Err(de::Error::duplicate_field("username"));
                            }
                            username = Some(map.next_value()?);
                        },
                        "password" => {
                            if password.is_some() {
                                return Err(de::Error::duplicate_field("password"));
                            }
                            password = Some(map.next_value()?);
                        },
                        _ => return Err(de::Error::unknown_field(&key, &["host", "username", "password"])),
                    }
                }

                let host = host.ok_or_else(|| de::Error::missing_field("host"))?;
                let username = username.ok_or_else(|| de::Error::missing_field("username"))?;
                let password = password.ok_or_else(|| de::Error::missing_field("password"))?;

                Ok(Data { host, username, password })
            }
        }

        deserializer.deserialize_map(DataVisitor)
    }
}


impl<'de> Deserialize<'de> for Server {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServerVisitor;

        impl<'de> Visitor<'de> for ServerVisitor {
            type Value = Server;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Server with field port")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut port = None;
                let mut host = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "port" {
                        if port.is_some() {
                            return Err(de::Error::duplicate_field("port"));
                        }
                        port = Some(map.next_value()?);
                    } else if key == "host" {
                        if host.is_some() {
                            return Err(de::Error::duplicate_field("host"));
                        }
                        host = Some(map.next_value()?);
                    } else {
                        return Err(de::Error::unknown_field(&key, &["port"]));
                    }
                }

                let port = port.ok_or_else(|| de::Error::missing_field("port"))?;
                let host = host.ok_or_else(|| de::Error::missing_field("host"))?;

                Ok(Server { port, host })
            }
        }

        deserializer.deserialize_map(ServerVisitor)
    }
}

use crate::ppm::registry::v1::storage::StorageBackend;
// Re exporting for easy use by 3rd parties
pub use crate::{
    ppm::{
        registry::v1::{Config, Local, S3, Server, Storage, Data},
        package::v1::{LockFile, LockedDependency, Manifest},
    },
    utils::fs::FileSystem
};