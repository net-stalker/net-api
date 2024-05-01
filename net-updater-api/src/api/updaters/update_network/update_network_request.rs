use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "update-network-request";

#[derive(Debug, PartialEq, Eq)]
pub struct UpdateNetworkRequestDTO {
    id: i64,
    name: String,
    color: String,
}
impl API for UpdateNetworkRequestDTO { }

impl UpdateNetworkRequestDTO {
    pub fn new(
        id: i64,
        name: &str,
        color: &str
    ) -> Self {
        UpdateNetworkRequestDTO {
            id,
            name: name.into(),
            color: color.into()
        }
    }
    
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }
}

impl Encoder for UpdateNetworkRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_i64(self.id).unwrap();

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("color");
        writer.write_string(&self.color).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for UpdateNetworkRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let name = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let color = binding.text();

        UpdateNetworkRequestDTO::new(
            id,
            name,
            color,
        )
    }
}

impl Typed for UpdateNetworkRequestDTO {
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

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;

    use crate::api::updaters::update_network::update_network_request::UpdateNetworkRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_update_network_request() {
        const ID: i64 = 1;
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let update_network_request = UpdateNetworkRequestDTO::new(
            ID,
            NAME,
            COLOR
        );
        let mut binary_user_reader = ReaderBuilder::new().build(update_network_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(NAME, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("color", binary_user_reader.field_name().unwrap());
        assert_eq!(COLOR, binary_user_reader.read_string().unwrap().text());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_update_network_request() {
        const ID: i64 = 1;
        const NAME: &str = "SOME_NAME";
        const COLOR: &str = "SOME_COLOR";
        let update_network_request = UpdateNetworkRequestDTO::new(
            ID,
            NAME,
            COLOR
        );
        
        assert_eq!(update_network_request, UpdateNetworkRequestDTO::decode(&update_network_request.encode()));
    }
}