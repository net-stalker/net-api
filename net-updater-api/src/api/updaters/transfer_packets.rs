use ion_rs;

use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use ion_rs::StreamItem;
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "transfer-packets-request";

#[derive(Debug, PartialEq, Eq)]
pub struct TransferPacketsRequestDTO {
    network_id: Option<String>,
    packets_ids: Vec<String>,
}
impl API for TransferPacketsRequestDTO { }

impl TransferPacketsRequestDTO {
    pub fn new(
        network_id: Option<&str>,
        packets_ids: &[String]
    ) -> Self {
        TransferPacketsRequestDTO {
            network_id: network_id.map(str::to_string),
            packets_ids: packets_ids.to_vec(),
        }
    }

    pub fn get_ids(&self) -> &[String] {
        &self.packets_ids
    }
}

impl Encoder for TransferPacketsRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("network_id");
        match self.network_id.as_deref() {
            Some(network_id) => writer.write_string(network_id).unwrap(),
            None => writer.write_null(IonType::String).unwrap(),
        }

        writer.set_field_name("packets_ids");
        writer.step_in(IonType::List).unwrap();
        self.packets_ids.iter().for_each(|id| writer.write_string(id).unwrap());
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for TransferPacketsRequestDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let network_id = match binary_user_reader.current() {
            StreamItem::Value(_) => Some(binary_user_reader.read_string().unwrap().text().to_string()),
            _ => None,
        };

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let ids: Vec<String> = binary_user_reader.read_all_elements().unwrap().iter_mut().map(|element| element.as_string().unwrap().to_string()).collect();

        TransferPacketsRequestDTO::new(network_id.as_deref(), &ids)
    }
}

impl Typed for TransferPacketsRequestDTO {
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

    use crate::api::updaters::transfer_packets::TransferPacketsRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_transfer_packets_request() {
        const NETWORK_ID: Option<&str> = Some("network_id");
        const PACKETS_IDS: [&'static str; 5] = ["1", "2", "3", "4", "5"];
        let transfer_packets_request = TransferPacketsRequestDTO::new(
            NETWORK_ID,
            &PACKETS_IDS.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
        );
        let mut binary_user_reader = ReaderBuilder::new().build(transfer_packets_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("network_id", binary_user_reader.field_name().unwrap());
        assert_eq!(NETWORK_ID.unwrap(), binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("packets_ids", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), PACKETS_IDS.len());
        for (element, core_id) in elements.iter().zip(PACKETS_IDS.as_slice()) {
            let id = element.as_string().unwrap();
            assert_eq!(id, *core_id);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_transfer_packets_request() {
        const NETWORK_ID: Option<&str> = Some("network_id");
        const PACKETS_IDS: [&'static str; 5] = ["1", "2", "3", "4", "5"];
        let transfer_packets_request = TransferPacketsRequestDTO::new(
            NETWORK_ID,
            &PACKETS_IDS.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
        );

        assert_eq!(transfer_packets_request, TransferPacketsRequestDTO::decode(&transfer_packets_request.encode()));
    }
}