use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "network-id-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NetworkIdRequestDTO {
    pub name: String,
    pub color: String,
}
impl API for NetworkIdRequestDTO { }

impl NetworkIdRequestDTO {
    pub fn new(name: &str, color: &str) -> Self {
        Self { 
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

impl Encoder for NetworkIdRequestDTO {
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

impl Decoder for NetworkIdRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> NetworkIdRequestDTO {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let name = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        let color = binary_user_reader.read_string().unwrap();
        
        NetworkIdRequestDTO::new(name.text(), color.text())
    }
}

impl Typed for NetworkIdRequestDTO {
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
    use net_core_api::core::typed_api::Typed;

    use crate::api::network::network_id_request::NetworkIdRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_network_id_request() {
        const NAME: &str = "name";
        const COLOR: &str = "color";
        let get_network_id_request = NetworkIdRequestDTO::new(NAME, COLOR);
        let mut binary_user_reader = ReaderBuilder::new().build(get_network_id_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(NAME, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("color", binary_user_reader.field_name().unwrap());
        assert_eq!(COLOR, binary_user_reader.read_string().unwrap().text());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_id_request() {
        const NAME: &str = "name";
        const COLOR: &str = "color";
        let get_network_id_request = NetworkIdRequestDTO::new(NAME, COLOR);
        assert_eq!(get_network_id_request, NetworkIdRequestDTO::decode(&get_network_id_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NAME: &str = "name";
        const COLOR: &str = "color";
        let get_network_id_request = NetworkIdRequestDTO::new(NAME, COLOR);
        assert_eq!(get_network_id_request.get_type(), NetworkIdRequestDTO::get_data_type());
        assert_eq!(get_network_id_request.get_type(), super::DATA_TYPE);
    }
}
