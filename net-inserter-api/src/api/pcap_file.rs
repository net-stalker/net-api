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


const DATA_TYPE: &str = "pcap_file";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PcapFileDTO {
    data: Vec<u8>
}

impl API for PcapFileDTO { }

impl InsertAPI for PcapFileDTO {
    fn into_insert_request(&self) -> InsertRequestDTO {
        InsertRequestDTO::new(
            DATA_TYPE,
            &self.encode()
        )
    }

    fn into_insert_entity(request: InsertRequestDTO) -> Result<PcapFileDTO, Box<dyn Error + Send + Sync>> {
        if request.get_entity_type() != DATA_TYPE {
            return Err(Box::new(DecodeInsertEntityError::WrongInsertRequest));
        }
        let pcap_file = PcapFileDTO::decode(request.get_entity());
        Ok(pcap_file)
    }
}

impl PcapFileDTO {
    pub fn new (data: &[u8]) -> Self {
        PcapFileDTO {
            data: data.into()
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl Encoder for PcapFileDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("data");
        writer.write_blob(&self.data).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for PcapFileDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let data = binding.as_slice();

        PcapFileDTO::new(
            data
        )
    }
}

impl Typed for PcapFileDTO {
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
    use crate::api::pcap_file::PcapFileDTO;

    #[test]
    fn reader_correctly_read_encoded_data_packet() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let pcap_file: PcapFileDTO = PcapFileDTO::new(DATA);
        let mut binary_user_reader = ReaderBuilder::new().build(pcap_file.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("data", binary_user_reader.field_name().unwrap());
        assert_eq!(DATA, binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    fn endec_data_packet() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let pcap_file: PcapFileDTO = PcapFileDTO::new(DATA);
        assert_eq!(pcap_file, PcapFileDTO::decode(&pcap_file.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let pcap_file: PcapFileDTO = PcapFileDTO::new(DATA);
        assert_eq!(pcap_file.get_type(), PcapFileDTO::get_data_type());
        assert_eq!(pcap_file.get_type(), super::DATA_TYPE);
    }

    #[test]
    fn test_insert_api() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let pcap_file: PcapFileDTO = PcapFileDTO::new(DATA);
        let insert_request = pcap_file.into_insert_request();
        let decoded_pcap_file = PcapFileDTO::into_insert_entity(insert_request).unwrap();
        assert_eq!(pcap_file, decoded_pcap_file);
    }

    #[test]
    fn test_pcap_file_with_insert_api_is_err() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let pcap_file: PcapFileDTO = PcapFileDTO::new(DATA);
        let insert_api = InsertRequestDTO::new("wrong_data_type", &pcap_file.encode());

        assert!(PcapFileDTO::into_insert_entity(insert_api).is_err());
    }
}