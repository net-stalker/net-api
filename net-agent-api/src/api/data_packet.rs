use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_proto_api::api::API;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


const DATA_TYPE: &str = "data_packet";

#[derive(Debug, PartialEq, Eq)]
pub struct DataPacketDTO {
    data: Vec<u8>
}
impl API for DataPacketDTO { }

impl DataPacketDTO {
    pub fn new (data: &[u8]) -> Self {
        DataPacketDTO {
            data: data.into()
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl Encoder for DataPacketDTO {
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

impl Decoder for DataPacketDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_blob().unwrap();
        let data = binding.as_slice();

        DataPacketDTO::new(
            data
        )
    }
}

impl net_proto_api::typed_api::Typed for DataPacketDTO {
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

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::typed_api::Typed;


    use crate::api::data_packet::DataPacketDTO;

    #[test]
    fn reader_correctly_read_encoded_data_packet() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let data_packet: DataPacketDTO = DataPacketDTO::new(DATA);
        let mut binary_user_reader = ReaderBuilder::new().build(data_packet.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("data", binary_user_reader.field_name().unwrap());
        assert_eq!(DATA, binary_user_reader.read_blob().unwrap().as_slice());
    }

    #[test]
    fn endec_data_packet() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let data_packet: DataPacketDTO = DataPacketDTO::new(DATA);
        assert_eq!(data_packet, DataPacketDTO::decode(&data_packet.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let data_packet: DataPacketDTO = DataPacketDTO::new(DATA);
        assert_eq!(data_packet.get_type(), DataPacketDTO::get_data_type());
        assert_eq!(data_packet.get_type(), super::DATA_TYPE);
    }
}