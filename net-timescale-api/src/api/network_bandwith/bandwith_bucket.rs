use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BandwithBucketDTO {
    bucket_timestamp: i64,
    total_bytes: i64,
}

impl BandwithBucketDTO {
    pub fn new (bucket_timestamp: i64, total_bytes: i64) -> Self {
        BandwithBucketDTO {
            bucket_timestamp,
            total_bytes,
        }
    }

    pub fn get_bucket_timestamp (&self) -> i64 {
        &self.bucket_timestamp
    }

    pub fn get_total_bytes (&self) -> i64 {
        &self.total_bytes
    }
}

impl Encoder for BandwithBucketDTO {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl Decoder for BandwithBucketDTO {
    fn decode(data: &[u8]) -> Self {
        todo!()
    }
}