use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.type_attribute(".", "#[derive(Hash, Eq)]");
    config.protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::configure().compile_with_config(
        config,
        &["../proto/commands_proto.proto"],
        &["../proto/"],
    )?;
    Ok(())
}
