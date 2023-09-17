pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "../protos/ppm/package/v1/manifest.proto",
                "../protos/ppm/package/v1/lock.proto",
                "../protos/ppm/registry/v1/server.proto",
                "../protos/ppm/registry/v1/config.proto",
                "../protos/ppm/registry/v1/storage.proto"
            ],
            &["../protos"],
        )?;
    Ok(())
}