use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::TextWriterBuilder;

use ion_rs::element::reader::ElementReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::api::API;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;


const DATA_TYPE: &str = "filter-entry";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FilterEntryDTO {
    endpoint: String,
    protocols: Vec<String>,
    bytes_rec: i64,
    bytes_sent: i64,
}
impl API for FilterEntryDTO { }

impl FilterEntryDTO {
    pub fn new(endpoint: &str, protocols: &[String], bytes_rec: i64, bytes_sent: i64) -> Self {
        FilterEntryDTO {
            endpoint: endpoint.into(),
            protocols: protocols.to_vec(),
            bytes_rec,
            bytes_sent,
        }
    }

    pub fn get_endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn get_protocols(&self) -> &[String] {
        self.protocols.as_slice()
    }

    pub fn get_bytes_received(&self) -> i64 {
        self.bytes_rec
    }

    pub fn get_bytes_sent(&self) -> i64 {
        self.bytes_sent
    }
}

impl Encoder for FilterEntryDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        #[cfg(feature = "ion-text")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("endpoint");
        writer.write_string(&self.endpoint).unwrap();

        writer.set_field_name("protocols");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for protocol in &self.protocols {
            writer.write_string(protocol).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("bytes_rec");
        writer.write_i64(self.bytes_rec).unwrap();

        writer.set_field_name("bytes_sent");
        writer.write_i64(self.bytes_sent).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for FilterEntryDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let endpoint = binding.text();
        
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut protocols = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            protocols.push(element.as_text().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        let bytes_rec = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let bytes_sent = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.step_out().unwrap();

        FilterEntryDTO::new(
            endpoint,
            protocols.as_slice(),
            bytes_rec,
            bytes_sent
        )
    }
}

impl Typed for FilterEntryDTO {
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
    use net_proto_api::typed_api::Typed;

    use crate::api::network_overview_dashboard_filters::filter_entry::FilterEntryDTO;

    #[test]
    fn reader_correctly_read_encoded_filter_entry() {
        const ENDPOINT: &str = "0.0.0.0:0000";
        let protocols: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];
        let bytes_rec = 1000;
        let bytes_sent = 500;
        let filter_entry = FilterEntryDTO::new(ENDPOINT, &protocols, bytes_rec, bytes_sent);
        let mut binary_user_reader = ReaderBuilder::new().build(filter_entry.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("endpoint", binary_user_reader.field_name().unwrap());
        assert_eq!(ENDPOINT,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("protocols", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let endeced_protocols: Vec<String> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
          element.as_string().unwrap().to_owned()
        }).collect();

        assert_eq!(protocols, endeced_protocols);

        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_rec", binary_user_reader.field_name().unwrap());
        assert_eq!(bytes_rec,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_sent", binary_user_reader.field_name().unwrap());
        assert_eq!(bytes_sent,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_filter_entry() {
        const ENDPOINT: &str = "0.0.0.0:0000";
        let protocols: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];
        let bytes_rec = 1000;
        let bytes_sent = 500;
        let filter_entry = FilterEntryDTO::new(ENDPOINT, &protocols, bytes_rec, bytes_sent);
        assert_eq!(filter_entry, FilterEntryDTO::decode(&filter_entry.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const ENDPOINT: &str = "0.0.0.0:0000";
        let protocols: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];
        let bytes_rec = 1000;
        let bytes_sent = 500;
        let filter_entry = FilterEntryDTO::new(ENDPOINT, &protocols, bytes_rec, bytes_sent);
        assert_eq!(filter_entry.get_type(), FilterEntryDTO::get_data_type());
        assert_eq!(filter_entry.get_type(), super::DATA_TYPE);
    }
}