use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/chat.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        // this till generate the server code for gRPC
        .build_server(true)
        // this will set the path for the path that contains encoded prost types, this is required for
        // implementing gRPC server reflection
        .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
        // set the output directory for the generate code
        .out_dir("src")
        // first argumetn is for the files we want to compile, second argumeent is the root location for these files
        .compile(&[proto_file], &["proto"])?;

    Ok(())
}
