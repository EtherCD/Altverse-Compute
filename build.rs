use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  napi_build::setup();

  let proto_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos");
  let proto_file = proto_dir.join("game.proto");

  println!("cargo:rerun-if-changed={}", proto_dir.display());
  println!("cargo:rerun-if-changed={}", proto_file.display());

  let mut config = prost_build::Config::new();

  config.compile_protos(&[proto_file], &[proto_dir])?;

  Ok(())
}
