use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_proto_api::api::API;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;

use super::bandwidth_bucket::BandwidthBucketDTO;


const DATA_TYPE: &str = "network_bandwidth";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkBandwidthDTO {
    bandwidth_buckets: Vec<BandwidthBucketDTO>,
}
impl API for NetworkBandwidthDTO { }

impl NetworkBandwidthDTO {
    pub fn new ( bandwidth_buckets: &[BandwidthBucketDTO],) -> Self {
        NetworkBandwidthDTO {
            bandwidth_buckets: bandwidth_buckets.to_vec(),
        }
    }

    pub fn get_bandwidth_buckets (&self) -> &[BandwidthBucketDTO] {
        &self.bandwidth_buckets
    }
}

impl Encoder for NetworkBandwidthDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("bandwidth_buckets");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.bandwidth_buckets.iter().for_each(|bandwidth_bucket| {
            let data = bandwidth_bucket.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthDTO {
    fn decode(data: &[u8]) -> Self {
        
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let bandwidth_bucket_elements = binary_user_reader.read_all_elements().unwrap();
        let mut endpoints = Vec::with_capacity(bandwidth_bucket_elements.len());
        bandwidth_bucket_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let endpoint = BandwidthBucketDTO::decode(data);
            endpoints.push(endpoint);
        });

        Self::new(
            &endpoints
        )
    }
}

impl Typed for NetworkBandwidthDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
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

    use crate::api::network_bandwidth::bandwidth_bucket::BandwidthBucketDTO;
    use crate::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
        const FIRST_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const FIRST_TOTAL_BYTES: i64 = i64::MAX;

        let first_bandwidth_bucket = BandwidthBucketDTO::new(
            FIRST_BUCKET_TIMESTAMP,
            FIRST_TOTAL_BYTES
        );

        const SECOND_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let second_bandwidth_bucket = BandwidthBucketDTO::new(
            SECOND_BUCKET_TIMESTAMP,
            SECOND_TOTAL_BYTES
        );

        let bandwidth_buckets = vec![
            first_bandwidth_bucket,
            second_bandwidth_bucket
        ];

        let network_bandwidth = NetworkBandwidthDTO::new(
            &bandwidth_buckets
        );

        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("bandwidth_buckets", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), bandwidth_buckets.len());
        for (element, bandwidth_bucket_core) in elements.iter().zip(bandwidth_buckets.as_slice()) {
            let encoded_endpoint = BandwidthBucketDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_endpoint, *bandwidth_bucket_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_bandwidth() {
        const FIRST_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const FIRST_TOTAL_BYTES: i64 = i64::MAX;

        let first_bandwidth_bucket = BandwidthBucketDTO::new(
            FIRST_BUCKET_TIMESTAMP,
            FIRST_TOTAL_BYTES
        );

        const SECOND_BUCKET_TIMESTAMP: i64 = i64::MAX;
        const SECOND_TOTAL_BYTES: i64 = i64::MAX;

        let second_bandwidth_bucket = BandwidthBucketDTO::new(
            SECOND_BUCKET_TIMESTAMP,
            SECOND_TOTAL_BYTES
        );

        let bandwidth_buckets = vec![
            first_bandwidth_bucket,
            second_bandwidth_bucket
        ];

        let network_bandwidth = NetworkBandwidthDTO::new(
            &bandwidth_buckets
        );

        assert_eq!(network_bandwidth, NetworkBandwidthDTO::decode(&network_bandwidth.encode()));
    }
}