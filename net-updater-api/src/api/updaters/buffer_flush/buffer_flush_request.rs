use ion_rs::{IonReader, IonType, IonWriter, ReaderBuilder};
use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "buffer-flush-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BufferFlushRequestDTO {
    delete: bool
}
impl API for BufferFlushRequestDTO { }

impl BufferFlushRequestDTO {
    pub fn new(delete: bool) -> Self {
        Self { delete }
    }

    pub fn is_delete(&self) -> bool {
        self.delete
    }
}

impl Encoder for BufferFlushRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("delete");
        writer.write_bool(self.delete).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for BufferFlushRequestDTO {
    fn decode(data: &[u8]) -> BufferFlushRequestDTO {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let delete = binary_user_reader.read_bool().unwrap();

        Self { delete }
    }
}

impl Typed for BufferFlushRequestDTO {
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

    use crate::api::updaters::buffer_flush::buffer_flush_request::BufferFlushRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_buffer_flush_requests() {
        const DELETE: bool = false;
        let update_network_request = BufferFlushRequestDTO::new(DELETE);
        let mut binary_user_reader = ReaderBuilder::new().build(update_network_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("delete", binary_user_reader.field_name().unwrap());
        assert_eq!(DELETE, binary_user_reader.read_bool().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_flush_buffer_request() {
        const DELETE: bool = false;
        let update_network_request = BufferFlushRequestDTO::new(DELETE);
        
        assert_eq!(update_network_request, BufferFlushRequestDTO::decode(&update_network_request.encode()));
    }
}