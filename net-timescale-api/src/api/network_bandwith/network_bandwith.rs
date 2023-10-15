use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

use super::bandwith_bucket::BandwithBucketDTO;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkBandwithDTO {
    bandwith_buckets: Vec<BandwithBucketDTO>,
}

impl NetworkBandwithDTO {
    pub fn new ( bandwith_buckets: &[BandwithBucketDTO],) -> Self {
        NetworkBandwithDTO {
            bandwith_buckets: bandwith_buckets.to_vec(),
        }
    }

    pub fn get_bandwith_buckets (&self) -> &[BandwithBucketDTO] {
        &self.bandwith_buckets
    }
}

impl Encoder for NetworkBandwithDTO {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}

impl Decoder for NetworkBandwithDTO {
    fn decode(data: &[u8]) -> Self {
        todo!()
    }
}