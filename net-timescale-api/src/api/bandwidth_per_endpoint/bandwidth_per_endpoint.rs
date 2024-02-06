use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use ion_rs::element::reader::ElementReader;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use crate::api::bandwidth_per_endpoint::endpoint::EndpointDTO;


const DATA_TYPE: &str = "bandwidth-per-endpoint";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BandwidthPerEndpointDTO {
    endpoints: Vec<EndpointDTO>,
}
impl API for  BandwidthPerEndpointDTO { }

impl BandwidthPerEndpointDTO {
    pub fn new(endpoints: &[EndpointDTO]) -> Self {
        Self { endpoints: endpoints.to_vec() }
    }

    fn m_new(endpoints: Vec<EndpointDTO>) -> Self {
        Self { endpoints }
    }

    pub fn get_endpoints(&self) -> &[EndpointDTO] {
        self.endpoints.as_slice()
    }
}

impl Encoder for BandwidthPerEndpointDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("endpoints");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.endpoints.iter().for_each(|endpoint| {
            let data = endpoint.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for BandwidthPerEndpointDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let endpoint_elements = binary_user_reader.read_all_elements().unwrap();
        let mut endpoints = Vec::with_capacity(endpoint_elements.len());
        endpoint_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let endpoint = EndpointDTO::decode(data);
            endpoints.push(endpoint);
        });

        Self::m_new(endpoints)
    }
}

impl Typed for BandwidthPerEndpointDTO {
    fn get_data_type() -> &'static str where Self: Sized {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::{IonReader, IonType, ReaderBuilder, StreamItem};
    use ion_rs::element::reader::ElementReader;

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::typed_api::Typed;

    use crate::api::bandwidth_per_endpoint::endpoint::EndpointDTO;
    use crate::api::bandwidth_per_endpoint::bandwidth_per_endpoint::BandwidthPerEndpointDTO;

    #[test]
    fn test_correctly_read_encoded_bandwidth_per_endpoint() {
        const ID1: &str = "id1";
        const ID2: &str = "id2";
        const ID3: &str = "id3";

        const BYTES_RECEIVED_1: i64 = 1;

        const BYTES_RECEIVED_2: i64 = 2;

        const BYTES_RECEIVED_3: i64 = 3;

        const BYTES_SENT_1: i64 = 10;

        const BYTES_SENT_2: i64 = 20;

        const BYTES_SENT_3: i64 = 30;

        let endpoints = vec![
            EndpointDTO::new(ID1, BYTES_RECEIVED_1, BYTES_SENT_1),
            EndpointDTO::new(ID2, BYTES_RECEIVED_2, BYTES_SENT_2),
            EndpointDTO::new(ID3, BYTES_RECEIVED_3, BYTES_SENT_3),
        ];

        let bandwidth_per_endpoint = BandwidthPerEndpointDTO::new(endpoints.as_slice());

        let mut binary_user_reader = ReaderBuilder::new().build(bandwidth_per_endpoint.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!(binary_user_reader.field_name().unwrap(), "endpoints");
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), endpoints.len());
        for (element, endpoint_core) in elements.iter().zip(endpoints.as_slice()) {
            let encoded_endpoint = EndpointDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_endpoint, *endpoint_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn test_endec_bandwidth_per_endpoint() {
        const ID1: &str = "id1";
        const ID2: &str = "id2";
        const ID3: &str = "id3";

        const BYTES_RECEIVED_1: i64 = 1;

        const BYTES_RECEIVED_2: i64 = 2;

        const BYTES_RECEIVED_3: i64 = 3;

        const BYTES_SENT_1: i64 = 10;

        const BYTES_SENT_2: i64 = 20;

        const BYTES_SENT_3: i64 = 30;

        let endpoints = vec![
            EndpointDTO::new(ID1, BYTES_RECEIVED_1, BYTES_SENT_1),
            EndpointDTO::new(ID2, BYTES_RECEIVED_2, BYTES_SENT_2),
            EndpointDTO::new(ID3, BYTES_RECEIVED_3, BYTES_SENT_3),
        ];

        let bandwidth_per_endpoint = BandwidthPerEndpointDTO::new(endpoints.as_slice());

        assert_eq!(bandwidth_per_endpoint, BandwidthPerEndpointDTO::decode(bandwidth_per_endpoint.encode().as_slice()));
    }

    #[test]
    fn test_getting_data_types() {
        const ID1: &str = "id1";
        const ID2: &str = "id2";
        const ID3: &str = "id3";

        const BYTES_RECEIVED_1: i64 = 1;

        const BYTES_RECEIVED_2: i64 = 2;

        const BYTES_RECEIVED_3: i64 = 3;

        const BYTES_SENT_1: i64 = 10;

        const BYTES_SENT_2: i64 = 20;

        const BYTES_SENT_3: i64 = 30;

        let endpoints = vec![
            EndpointDTO::new(ID1, BYTES_RECEIVED_1, BYTES_SENT_1),
            EndpointDTO::new(ID2, BYTES_RECEIVED_2, BYTES_SENT_2),
            EndpointDTO::new(ID3, BYTES_RECEIVED_3, BYTES_SENT_3),
        ];

        let bandwidth_per_endpoint = BandwidthPerEndpointDTO::new(endpoints.as_slice());
        assert_eq!(bandwidth_per_endpoint.get_type(), BandwidthPerEndpointDTO::get_data_type());
        assert_eq!(bandwidth_per_endpoint.get_type(), super::DATA_TYPE);
    }
}