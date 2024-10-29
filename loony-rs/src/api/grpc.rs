//! This module wraps underlying GRPC stubs and organizes them into nested module structure.
macro_rules! include_proto {
    ($package: tt) => {
        include!(concat!("../grpc_stubs/", concat!($package, ".rs")));
    };
}

pub mod google {

    pub mod rpc {
        include_proto!("google.rpc");
    }

    pub mod longrunning {
        include_proto!("google.longrunning");
    }

    pub mod cloud {

        pub mod speechtotext {

            pub mod v1 {
                #[cfg(any(feature = "default", feature = "google-cloud-speechtotext-v1",))]
                include_proto!("google.cloud.speech.v1");
            }

            pub mod v1p1beta1 {
                #[cfg(any(feature = "default", feature = "google-cloud-speechtotext-v1p1beta1",))]
                include_proto!("google.cloud.speech.v1p1beta1");
            }
        }
    }
}
