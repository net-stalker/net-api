use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;


#[derive(Debug, PartialEq, Eq)]
pub struct DataPacketDTO {
    data: Vec<u8>
}

impl DataPacketDTO {
    pub fn new (data: &[u8]) -> Self {
        DataPacketDTO {
            data: data.into()
        }
    }

    pub fn get_data (&self) -> &[u8] {
        &self.data
    }
}

impl Encoder for DataPacketDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        #[cfg(feature = "ion-text")]
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("data");
        writer.write_blob(&self.data).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for DataPacketDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
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


#[cfg(test)]
mod tests {
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;

    
    use crate::api::data_packet::DataPacketDTO;

    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
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
    fn endec_graph_edge() {
        const DATA: &[u8] = "SOME_RAW_PCAP".as_bytes();
        let data_packet: DataPacketDTO = DataPacketDTO::new(DATA);
        assert_eq!(data_packet, DataPacketDTO::decode(&data_packet.encode()));
    }
}