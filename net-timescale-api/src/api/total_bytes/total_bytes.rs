use ion_rs;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;
use crate::api::total_bytes::endpoint::EndpointDTO;


const DATA_TYPE: &str = "total-bytes";
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TotalBytesDTO {
    endpoints: Vec<EndpointDTO>,
}

impl TotalBytesDTO {
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

impl Encoder for TotalBytesDTO {
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

        writer.set_field_name("endpoints");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
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

impl Decoder for TotalBytesDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
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

impl Typed for TotalBytesDTO {
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
    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::typed_api::Typed;
    use crate::api::total_bytes::endpoint::EndpointDTO;
    use crate::api::total_bytes::total_bytes::TotalBytesDTO;

    #[test]
    fn test_correctly_read_encoded_total_bytes() {
        const ID1: &str = "id1";
        const ID2: &str = "id2";
        const ID3: &str = "id3";

        const BR1: i64 = 1;

        const BR2: i64 = 2;

        const BR3: i64 = 3;

        const BS1: i64 = 10;

        const BS2: i64 = 20;

        const BS3: i64 = 30;

        let endpoints = vec![
            EndpointDTO::new(ID1, BR1, BS1),
            EndpointDTO::new(ID2, BR2, BS2),
            EndpointDTO::new(ID3, BR3, BS3),
        ];

        let total_bytes = TotalBytesDTO::new(endpoints.as_slice());

        let mut binary_user_reader = ReaderBuilder::new().build(total_bytes.encode()).unwrap();

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
    fn test_endec_total_bytes() {
        const ID1: &str = "id1";
        const ID2: &str = "id2";
        const ID3: &str = "id3";

        const BR1: i64 = 1;

        const BR2: i64 = 2;

        const BR3: i64 = 3;

        const BS1: i64 = 10;

        const BS2: i64 = 20;

        const BS3: i64 = 30;

        let endpoints = vec![
            EndpointDTO::new(ID1, BR1, BS1),
            EndpointDTO::new(ID2, BR2, BS2),
            EndpointDTO::new(ID3, BR3, BS3),
        ];

        let total_bytes = TotalBytesDTO::new(endpoints.as_slice());

        assert_eq!(total_bytes, TotalBytesDTO::decode(total_bytes.encode().as_slice()));
    }

    #[test]
    fn test_getting_data_types() {
        const ID1: &str = "id1";
        const ID2: &str = "id2";
        const ID3: &str = "id3";

        const BR1: i64 = 1;

        const BR2: i64 = 2;

        const BR3: i64 = 3;

        const BS1: i64 = 10;

        const BS2: i64 = 20;

        const BS3: i64 = 30;

        let endpoints = vec![
            EndpointDTO::new(ID1, BR1, BS1),
            EndpointDTO::new(ID2, BR2, BS2),
            EndpointDTO::new(ID3, BR3, BS3),
        ];

        let total_bytes = TotalBytesDTO::new(endpoints.as_slice());
        assert_eq!(total_bytes.get_type(), TotalBytesDTO::get_data_type());
        assert_eq!(total_bytes.get_type(), super::DATA_TYPE);
    }
}