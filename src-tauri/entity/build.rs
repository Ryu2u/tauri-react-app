use std::io::Result;

fn main() -> Result<()> {
  let mut config = prost_build::Config::new();
  config.out_dir("src/proto");
  config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
  config.compile_protos(&["src/proto/message.proto"], &["src/proto"])?;
  // prost_build::compile_protos(&["src/proto/response.proto"], &["src/proto"])?;
  Ok(())
}
