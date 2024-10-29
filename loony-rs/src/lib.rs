/// Google trusted certificates. Used by underlying Rust/Tonic gRPC services when establishing gRPC connection/channel.
pub const CERTIFICATES: &[u8] = include_bytes!("../../res/certs/roots.pem");

pub mod api;
pub mod common;
pub mod errors;
pub mod speechtotext;
