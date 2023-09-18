pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "../protos/plm/package/v1/manifest.proto",
                "../protos/plm/package/v1/lock.proto",
                "../protos/plm/registry/v1/server.proto",
                "../protos/plm/registry/v1/config.proto",
                "../protos/plm/registry/v1/storage.proto",
                "../protos/plm/registry/v1/registry.proto",
                "../protos/plm/library/v1/library.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}
