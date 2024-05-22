use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "network";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkDTO {
    id: String,
    name: String,
    color: String,
}
impl API for NetworkDTO { }

impl NetworkDTO {
    pub fn new(
        id: &str,
        name: &str,
        color: &str,
    ) -> Self {
        NetworkDTO { 
            id: id.into(),
            name: name.into(),
            color: color.into(),
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }
}

impl Encoder for NetworkDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_string(&self.id).unwrap();
        
        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("color");
        writer.write_string(&self.color).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_string().unwrap();

        binary_user_reader.next().unwrap();
        let name = binary_user_reader.read_string().unwrap();
        
        binary_user_reader.next().unwrap();
        let color = binary_user_reader.read_string().unwrap();

        NetworkDTO::new(
            id.text(),
            name.text(), 
            color.text(),
        )
    }
}

impl Typed for NetworkDTO {
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
    use crate::api::network::network::NetworkDTO;

    #[test]
    fn reader_correctly_read_encoded_network() {
        const NETWORK_ID: &str = "network_id";
        const NAME: &str = "my network name";
        const COLOR: &str = "color";
        let network = NetworkDTO::new(
            NETWORK_ID,
            NAME,
            COLOR,
        );
        let mut binary_user_reader = ReaderBuilder::new().build(network.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_ID, binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(NAME, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("color", binary_user_reader.field_name().unwrap());
        assert_eq!(COLOR, binary_user_reader.read_string().unwrap().text());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network() {
        const NETWORK_ID: &str = "network_id";
        const NAME: &str = "my network name";
        const COLOR: &str = "color";
        let network = NetworkDTO::new(
            NETWORK_ID,
            NAME,
            COLOR,
        );
        assert_eq!(network, NetworkDTO::decode(&network.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NETWORK_ID: &str = "network_id";
        const NAME: &str = "my network name";
        const COLOR: &str = "color";
        let network = NetworkDTO::new(
            NETWORK_ID,
            NAME,
            COLOR,
        );
        assert_eq!(network.get_type(), NetworkDTO::get_data_type());
        assert_eq!(network.get_type(), super::DATA_TYPE);
    }
}