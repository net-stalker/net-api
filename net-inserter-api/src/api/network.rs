use std::error::Error;

use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use super::core::error::DecodeInsertEntityError;
use super::core::insert_api::InsertAPI;
use super::insert_request::InsertRequestDTO;


const DATA_TYPE: &str = "insert-network-request";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InsertNetworkRequestDTO {
    name: String,
    color: String,
}

impl API for InsertNetworkRequestDTO { }

impl InsertAPI for InsertNetworkRequestDTO {
    fn into_insert_request(self) -> InsertRequestDTO {
        InsertRequestDTO::new(
            DATA_TYPE,
            &self.encode()
        )
    }

    fn into_insert_entity(request: InsertRequestDTO) -> Result<InsertNetworkRequestDTO, Box<dyn Error + Send + Sync>> {
        if request.get_entity_type() != DATA_TYPE {
            return Err(Box::new(DecodeInsertEntityError::WrongInsertRequest));
        }
        let insert_network_request = InsertNetworkRequestDTO::decode(request.get_entity());
        Ok(insert_network_request)
    }
}

impl InsertNetworkRequestDTO {
    pub fn new(name: &str, color: &str) -> Self {
        InsertNetworkRequestDTO {
            name: name.to_string(),
            color: color.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }
}

impl Encoder for InsertNetworkRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("color");
        writer.write_string(&self.color).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for InsertNetworkRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let name = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let color = binding.text();

        InsertNetworkRequestDTO::new(
            name,
            color,
        )
    }
}

impl Typed for InsertNetworkRequestDTO {
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

    use net_core_api::core::decoder_api::Decoder;
    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::typed_api::Typed;


    use crate::api::core::insert_api::InsertAPI;
    use crate::api::insert_request::InsertRequestDTO;
    use crate::api::network::InsertNetworkRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_insert_network_request() {
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let insert_network_request: InsertNetworkRequestDTO = InsertNetworkRequestDTO::new(NAME, COLOR);
        let mut binary_user_reader = ReaderBuilder::new().build(insert_network_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(NAME, binary_user_reader.read_string().unwrap());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("color", binary_user_reader.field_name().unwrap());
        assert_eq!(COLOR, binary_user_reader.read_string().unwrap());
    }

    #[test]
    fn endec_insert_network_request() {
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let insert_network_request: InsertNetworkRequestDTO = InsertNetworkRequestDTO::new(NAME, COLOR);
        assert_eq!(insert_network_request, InsertNetworkRequestDTO::decode(&insert_network_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let insert_network_request: InsertNetworkRequestDTO = InsertNetworkRequestDTO::new(NAME, COLOR);
        assert_eq!(insert_network_request.get_type(), InsertNetworkRequestDTO::get_data_type());
        assert_eq!(insert_network_request.get_type(), super::DATA_TYPE);
    }

    #[test]
    fn test_insert_api() {
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let insert_network_request: InsertNetworkRequestDTO = InsertNetworkRequestDTO::new(NAME, COLOR);
        let insert_request = insert_network_request.clone().into_insert_request();
        let decoded_insert_network_request = InsertNetworkRequestDTO::into_insert_entity(insert_request).unwrap();
        assert_eq!(insert_network_request, decoded_insert_network_request);
    }

    #[test]
    fn test_insert_network_request_with_insert_api_is_err() {
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let insert_network_request: InsertNetworkRequestDTO = InsertNetworkRequestDTO::new(NAME, COLOR);
        let insert_api = InsertRequestDTO::new("wrong_data_type", &insert_network_request.encode());

        assert!(InsertNetworkRequestDTO::into_insert_entity(insert_api).is_err());
    }
}