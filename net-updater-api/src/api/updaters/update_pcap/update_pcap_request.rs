use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use ion_rs::StreamItem;
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "update-network-request";

#[derive(Debug, PartialEq, Eq)]
pub struct UpdatePacketRequestDTO {
    id: String,
    network_id: Option<String>,
}
impl API for UpdatePacketRequestDTO { }

impl UpdatePacketRequestDTO {
    pub fn new(
        id: &str,
        network_id: Option<&str>,
    ) -> Self {
        UpdatePacketRequestDTO {
            id: id.into(),
            network_id: network_id.map(str::to_string),
        }
    }
    
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_network_id(&self) -> Option<&str> {
        self.network_id.as_deref()
    }
}

impl Encoder for UpdatePacketRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_string(&self.id).unwrap();

        writer.set_field_name("network_id");
        match self.network_id.as_deref() {
            Some(network_id) => writer.write_string(network_id).unwrap(),
            None => writer.write_null(IonType::String).unwrap(),
        }

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for UpdatePacketRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let id = binding.text();
        
        binary_user_reader.next().unwrap();
        let network_id = match binary_user_reader.current() {
            StreamItem::Value(_) => Some(binary_user_reader.read_string().unwrap().text().to_string()),
            _ => None,
        };

        UpdatePacketRequestDTO::new(
            id,
            network_id.as_deref(),
        )
    }
}

impl Typed for UpdatePacketRequestDTO {
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

    use crate::api::updaters::update_pcap::update_pcap_request::UpdatePacketRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_update_pcap_request() {
        const ID: &str = "1";
        const NETWORK_ID: Option<&str> = Some("2");
        let update_pcap_request = UpdatePacketRequestDTO::new(
            ID,
            NETWORK_ID
        );
        let mut binary_user_reader = ReaderBuilder::new().build(update_pcap_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("network_id", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_ID.unwrap(), binary_user_reader.read_string().unwrap().text());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_update_pcap_request_some_network() {
        const ID: &str = "1";
        const NETWORK_ID: Option<&str> = Some("2");
        let update_pcap_request = UpdatePacketRequestDTO::new(
            ID,
            NETWORK_ID
        );
        
        assert_eq!(update_pcap_request, UpdatePacketRequestDTO::decode(&update_pcap_request.encode()));
    }

    #[test]
    fn endec_update_pcap_request_none_network() {
        const ID: &str = "1";
        const NETWORK_ID: Option<&str> = Some("2");
        let update_pcap_request = UpdatePacketRequestDTO::new(
            ID,
            NETWORK_ID
        );
        
        assert_eq!(update_pcap_request, UpdatePacketRequestDTO::decode(&update_pcap_request.encode()));
    }
}