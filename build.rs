fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "rpc")]
    if cfg!(feature = "rpc") {
        capnpc::CompilerCommand::new()
            .src_prefix("protobufs")
            .file("protobufs/hello_world.capnp")
            .output_path("src/protobufs")
            .run()?;
    }
    Ok(())
}
