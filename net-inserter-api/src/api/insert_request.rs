use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonWriter;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "insert_request";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InsertRequestDTO {
    entity_type: String,
    entity: Vec<u8>,
}

impl API for InsertRequestDTO {}

impl InsertRequestDTO {
    pub fn new(entity_type: &str, entity: &[u8]) -> Self {
        InsertRequestDTO {
            entity_type: entity_type.into(),
            entity: entity.into()
        }
    }

    pub fn get_entity_type(&self) -> &str {
        &self.entity_type
    }

    pub fn get_entity(&self) -> &[u8] {
        &self.entity
    }
}

impl Typed for InsertRequestDTO {
    fn get_type(&self) -> &str {
        DATA_TYPE
    }

    fn get_data_type() -> &'static str where Self : Sized {
        DATA_TYPE
    }
}

impl Encoder for InsertRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("entity_type");
        writer.write_string(&self.entity_type).unwrap();

        writer.set_field_name("entity");
        writer.write_blob(&self.entity).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl Decoder for InsertRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let entity_type = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let entity = binding.as_slice();

        InsertRequestDTO::new(
            entity_type,
            entity,
        )
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::{IonType, ReaderBuilder, StreamItem};

    use super::*;


    #[test]
    fn reader_correctly_read_encoded_envelope() {
        const ENTITY_TYPE: &str = "ENTITY_TYPE";
        const ENTITY: &[u8] = "ENTITY".as_bytes();
        let insert_request = InsertRequestDTO::new(ENTITY_TYPE, ENTITY);

        let mut binary_user_reader = ReaderBuilder::new().build(insert_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("entity_type", binary_user_reader.field_name().unwrap());
        assert_eq!(ENTITY_TYPE, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("entity", binary_user_reader.field_name().unwrap());
        assert_eq!(ENTITY, binary_user_reader.read_blob().unwrap().as_slice());
    }
    
    #[test]
    fn endec_envelope() {
        const ENTITY_TYPE: &str = "ENTITY_TYPE";
        const ENTITY: &[u8] = "ENTITY".as_bytes();
        let insert_request = InsertRequestDTO::new(ENTITY_TYPE, ENTITY);
        assert_eq!(insert_request, InsertRequestDTO::decode(&insert_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const ENTITY_TYPE: &str = "ENTITY_TYPE";
        const ENTITY: &[u8] = "ENTITY".as_bytes();
        let insert_request = InsertRequestDTO::new(ENTITY_TYPE, ENTITY);
        
        assert_eq!(insert_request.get_type(), InsertRequestDTO::get_data_type());
        assert_eq!(insert_request.get_type(), super::DATA_TYPE);
    }
}