#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub files: ::prost::alloc::vec::Vec<File>,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
/// Manifest holds the metadata for a Protobuf package in the registry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manifest {
    /// Name of the package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Version of the package.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// A brief description of the package.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Points to a directory within your project where the .proto files are stored
    #[prost(string, tag = "4")]
    pub src_dir: ::prost::alloc::string::String,
    /// The repository URL where the package source can be found.
    #[prost(string, tag = "5")]
    pub repository_url: ::prost::alloc::string::String,
    /// License under which the package is distributed.
    #[prost(string, tag = "6")]
    pub license: ::prost::alloc::string::String,
    /// Names of the authors or maintainers.
    #[prost(string, repeated, tag = "7")]
    pub authors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of dependencies.
    #[prost(map = "string, string", tag = "8")]
    pub dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Custom metadata in key-value pairs.
    #[prost(map = "string, string", tag = "9")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// LockedDependency contains locked-down information about a dependency.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDependency {
    /// The locked version of the dependency.
    #[prost(string, tag = "1")]
    pub locked_version: ::prost::alloc::string::String,
    /// The source from which this package version comes (e.g., URL or file path).
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// Optional checksum for package integrity verification.
    #[prost(string, tag = "3")]
    pub checksum: ::prost::alloc::string::String,
}
/// LockFile holds the locked-down versions of dependencies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockFile {
    /// A map of package names to their locked versions and sources.
    #[prost(map = "string, message", tag = "1")]
    pub locked_dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        LockedDependency,
    >,
    /// A hash or checksum for integrity verification, if applicable.
    #[prost(string, tag = "2")]
    pub integrity_hash: ::prost::alloc::string::String,
}
