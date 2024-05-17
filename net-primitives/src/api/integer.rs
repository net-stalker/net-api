use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonWriter;

use net_core_api::core::api::API;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "integer";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer {
    value: i64,
}

impl API for Integer {}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl Typed for Integer {
    fn get_type(&self) -> &str {
        DATA_TYPE
    }

    fn get_data_type() -> &'static str where Self : Sized {
        DATA_TYPE
    }
}

impl Encoder for Integer {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("value");
        writer.write_i64(self.value).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().to_owned()
    }
}

impl Decoder for Integer {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let value = binary_user_reader.read_i64().unwrap();

        Integer::new(value)
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


    use super::Integer;


    #[test]
    fn reader_correctly_read_encoded_envelope() {
        const INTEGER_VALUE: i64 = 228;
        let integer = Integer::new(INTEGER_VALUE);

        let mut binary_user_reader = ReaderBuilder::new().build(integer.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("value", binary_user_reader.field_name().unwrap());
        assert_eq!(INTEGER_VALUE,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }
    
    #[test]

    fn endec_envelope() {
        const INTEGER_VALUE: i64 = 228;
        let integer = Integer::new(INTEGER_VALUE);
        assert_eq!(integer, Integer::decode(&integer.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const INTEGER_VALUE: i64 = 228;
        let integer = Integer::new(INTEGER_VALUE);
        
        assert_eq!(integer.get_type(), Integer::get_data_type());
        assert_eq!(integer.get_type(), super::DATA_TYPE);
    }
}