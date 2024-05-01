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

const DATA_TYPE: &str = "update-pcap-request";

#[derive(Debug, PartialEq, Eq)]
pub struct UpdatePcapRequestDTO {
    id: i64,
    network_id: Option<i64>,
}
impl API for UpdatePcapRequestDTO { }

impl UpdatePcapRequestDTO {
    pub fn new(
        id: i64,
        network_id: Option<i64>,
    ) -> Self {
        UpdatePcapRequestDTO {
            id,
            network_id,
        }
    }
    
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_network_id(&self) -> Option<i64> {
        self.network_id
    }
}

impl Encoder for UpdatePcapRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("id");
        writer.write_i64(self.id).unwrap();

        writer.set_field_name("network_id");
        match self.network_id {
            Some(network_id) => writer.write_i64(network_id).unwrap(),
            None => writer.write_null(IonType::Int).unwrap(),
        }

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for UpdatePcapRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let id = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let network_id = match binary_user_reader.current() {
            StreamItem::Value(_) => Some(binary_user_reader.read_i64().unwrap()),
            _ => None,
        };

        UpdatePcapRequestDTO::new(
            id,
            network_id,
        )
    }
}

impl Typed for UpdatePcapRequestDTO {
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

    use crate::api::updaters::update_pcap::update_pcap_request::UpdatePcapRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_update_pcap_request() {
        const ID: i64 = 1;
        const NETWORK_ID: Option<i64> = Some(2);
        let update_pcap_request = UpdatePcapRequestDTO::new(
            ID,
            NETWORK_ID
        );
        let mut binary_user_reader = ReaderBuilder::new().build(update_pcap_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("id", binary_user_reader.field_name().unwrap());
        assert_eq!(ID, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("network_id", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_ID.unwrap(), binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_update_pcap_request_some_network() {
        const ID: i64 = 1;
        const NETWORK_ID: Option<i64> = Some(2);
        let update_pcap_request = UpdatePcapRequestDTO::new(
            ID,
            NETWORK_ID
        );
        
        assert_eq!(update_pcap_request, UpdatePcapRequestDTO::decode(&update_pcap_request.encode()));
    }

    #[test]
    fn endec_update_pcap_request_none_network() {
        const ID: i64 = 1;
        const NETWORK_ID: Option<i64> = None;
        let update_pcap_request = UpdatePcapRequestDTO::new(
            ID,
            NETWORK_ID
        );
        
        assert_eq!(update_pcap_request, UpdatePcapRequestDTO::decode(&update_pcap_request.encode()));
    }
}