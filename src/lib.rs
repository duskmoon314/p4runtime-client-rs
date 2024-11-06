//! p4runtime-client: A P4Runtime client wrapper crate

#![deny(missing_docs)]

pub mod client;
pub mod config;
pub mod counter;
pub mod digest;
pub mod p4info;
pub mod table;
pub mod utils;

pub use p4runtime;

#[allow(missing_docs)]
pub mod error {
    error_set::error_set! {
        TonicStatus = {
            Status(tonic::Status)
        };
        TokioError = {
            MpscSendError(tokio::sync::mpsc::error::SendError<p4runtime::p4::v1::StreamMessageRequest>),
        };
        ClientError = {
            #[display("Please connect to server first")]
            MissingP4rtClient,
            Timeout,
            ArbitrationFailed,
            NoneEntity,
            MultipleEntities {
                n: usize,
            },
            UnexpectedEntry
        } || TonicStatus || TokioError;
    }
}
