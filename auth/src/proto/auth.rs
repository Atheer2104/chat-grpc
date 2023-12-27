include!("../authentication.rs");

// used for server reflection
pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("store_descriptor");
