use ion_rs;

use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "delete-packets-request";

#[derive(Debug, PartialEq, Eq)]
pub struct DeletePacketsRequestDTO {
    ids: Vec<String>,
}
impl API for DeletePacketsRequestDTO { }

impl DeletePacketsRequestDTO {
    pub fn new(
        ids: &[String]
    ) -> Self {
        DeletePacketsRequestDTO {
            ids: ids.to_vec(),
        }
    }
    
    pub fn get_ids(&self) -> &[String] {
        &self.ids
    }
}

impl Encoder for DeletePacketsRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("ids");
        writer.step_in(IonType::List).unwrap();
        self.ids.iter().for_each(|id| writer.write_string(id).unwrap());
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for DeletePacketsRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let ids: Vec<String> = binary_user_reader.read_all_elements().unwrap().iter_mut().map(|element| element.as_string().unwrap().to_string()).collect();
        
        DeletePacketsRequestDTO::new(&ids)
    }
}

impl Typed for DeletePacketsRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}

#[cfg(test)]
mod tests {
    use ion_rs::element::reader::ElementReader;
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;

    use crate::api::packets::DeletePacketsRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_delete_packets_request() {
        const IDS: [&'static str; 5] = ["1", "2", "3", "4", "5"];
        let delete_packets_request = DeletePacketsRequestDTO::new(
            &IDS.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
        );
        let mut binary_user_reader = ReaderBuilder::new().build(delete_packets_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("ids", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), IDS.len());
        for (element, core_id) in elements.iter().zip(IDS.as_slice()) {
            let id = element.as_string().unwrap();
            assert_eq!(id, *core_id);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_delete_packets_request() {
        const IDS: [&'static str; 5] = ["1", "2", "3", "4", "5"];
        let delete_packets_request = DeletePacketsRequestDTO::new(
            &IDS.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
        );
        
        assert_eq!(delete_packets_request, DeletePacketsRequestDTO::decode(&delete_packets_request.encode()));
    }
}
