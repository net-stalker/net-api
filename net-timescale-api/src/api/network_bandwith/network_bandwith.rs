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
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        #[cfg(feature = "ion-text")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("bandwith_buckets");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        self.bandwith_buckets.iter().for_each(|bandwith_bucket| {
            let data = bandwith_bucket.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwithDTO {
    fn decode(data: &[u8]) -> Self {
        
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let bandwith_bucket_elements = binary_user_reader.read_all_elements().unwrap();
        let mut endpoints = Vec::with_capacity(bandwith_bucket_elements.len());
        bandwith_bucket_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let endpoint = BandwithBucketDTO::decode(data);
            endpoints.push(endpoint);
        });

        Self::new(
            &endpoints
        )
    }
}

#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use ion_rs::element::reader::ElementReader;
    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;

    use crate::api::network_bandwith::bandwith_bucket::BandwithBucketDTO;
    use crate::api::network_bandwith::network_bandwith::NetworkBandwithDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
        const FIRST_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const FIRST_TOTAL_BYTES: i64 = i64::MAX;

        let first_bandwith_bucket = BandwithBucketDTO::new(
            FIRST_BUCKET_TIMESTAMP,
            FIRST_TOTAL_BYTES
        );

        const SECOND_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let second_bandwith_bucket = BandwithBucketDTO::new(
            SECOND_BUCKET_TIMESTAMP,
            SECOND_TOTAL_BYTES
        );

        let bandwith_buckets = vec![
            first_bandwith_bucket,
            second_bandwith_bucket
        ];

        let network_bandwith = NetworkBandwithDTO::new(
            &bandwith_buckets
        );

        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwith.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("bandwith_buckets", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), bandwith_buckets.len());
        for (element, bandwith_bucket_core) in elements.iter().zip(bandwith_buckets.as_slice()) {
            let encoded_endpoint = BandwithBucketDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_endpoint, *bandwith_bucket_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_bandwith() {
        const FIRST_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const FIRST_TOTAL_BYTES: i64 = i64::MAX;

        let first_bandwith_bucket = BandwithBucketDTO::new(
            FIRST_BUCKET_TIMESTAMP,
            FIRST_TOTAL_BYTES
        );

        const SECOND_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let second_bandwith_bucket = BandwithBucketDTO::new(
            SECOND_BUCKET_TIMESTAMP,
            SECOND_TOTAL_BYTES
        );

        let bandwith_buckets = vec![
            first_bandwith_bucket,
            second_bandwith_bucket
        ];

        let network_bandwith = NetworkBandwithDTO::new(
            &bandwith_buckets
        );

        assert_eq!(network_bandwith, NetworkBandwithDTO::decode(&network_bandwith.encode()));
    }
}