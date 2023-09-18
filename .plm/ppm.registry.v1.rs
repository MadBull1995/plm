#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S3 {
    /// The `Simple Storage Service` (s3) bucket name
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Local {
    /// The full path to the local directory to store libraries in
    #[prost(string, tag = "1")]
    pub registry_path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Storage {
    #[prost(oneof = "storage::StorageBackend", tags = "1, 2")]
    pub storage_backend: ::core::option::Option<storage::StorageBackend>,
}
/// Nested message and enum types in `Storage`.
pub mod storage {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StorageBackend {
        #[prost(message, tag = "1")]
        Local(super::Local),
        #[prost(message, tag = "2")]
        S3(super::S3),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialDownloadRequest {
    /// The library full name e.g: 'some_package' / '@org/some_package'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of full 'library/package' names
    #[prost(string, repeated, tag = "2")]
    pub packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadRequest {
    #[prost(enumeration = "Compressions", tag = "3")]
    pub compression: i32,
    #[prost(oneof = "download_request::FullOrPartial", tags = "1, 2")]
    pub full_or_partial: ::core::option::Option<download_request::FullOrPartial>,
}
/// Nested message and enum types in `DownloadRequest`.
pub mod download_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FullOrPartial {
        #[prost(string, tag = "1")]
        Full(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        Partial(super::PartialDownloadRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadResponse {
    #[prost(oneof = "download_response::ProtobufOrGz", tags = "1, 2")]
    pub protobuf_or_gz: ::core::option::Option<download_response::ProtobufOrGz>,
}
/// Nested message and enum types in `DownloadResponse`.
pub mod download_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProtobufOrGz {
        #[prost(message, tag = "1")]
        Protobuf(super::super::super::library::v1::Library),
        #[prost(bytes, tag = "2")]
        Gz(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Compressions {
    Protobuf = 0,
    Gz = 1,
}
impl Compressions {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Compressions::Protobuf => "PROTOBUF",
            Compressions::Gz => "GZ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROTOBUF" => Some(Self::Protobuf),
            "GZ" => Some(Self::Gz),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Server {
    #[prost(uint32, tag = "1")]
    pub port: u32,
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(message, optional, tag = "1")]
    pub storage: ::core::option::Option<Storage>,
    #[prost(message, optional, tag = "2")]
    pub server: ::core::option::Option<Server>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<Data>,
}
