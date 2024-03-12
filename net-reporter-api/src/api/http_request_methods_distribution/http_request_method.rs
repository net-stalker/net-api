use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

const DATA_TYPE: &str = "http_request_method";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpRequestMethodDTO {
    name: String,
    amount: i64,
}
impl API for HttpRequestMethodDTO { }

impl HttpRequestMethodDTO {
    pub fn new(name: &str, amount: i64) -> Self {
        HttpRequestMethodDTO {
            name: name.into(),
            amount,
        }
    }

    pub fn get_method_name(&self) -> &str {
        &self.name
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }
}

impl Encoder for HttpRequestMethodDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("amount");
        writer.write_i64(self.amount).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpRequestMethodDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let name = binding.text();

        binary_user_reader.next().unwrap();
        let amount = binary_user_reader.read_i64().unwrap();

        binary_user_reader.step_out().unwrap();

        HttpRequestMethodDTO::new(name, amount)
    }
}

impl Typed for HttpRequestMethodDTO {
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
    
    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::typed_api::Typed;

    use crate::api::http_request_methods_distribution::http_request_method::HttpRequestMethodDTO;

    #[test]
    fn reader_correctly_read_encoded_http_request() {
        const METHOD_NAME: &str = "GET";
        let total_amount = 1000;
        let http_request = HttpRequestMethodDTO::new(METHOD_NAME, total_amount);
        let mut binary_user_reader = ReaderBuilder::new().build(http_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("name", binary_user_reader.field_name().unwrap());
        assert_eq!(METHOD_NAME,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("amount", binary_user_reader.field_name().unwrap());
        assert_eq!(total_amount,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_request() {
        const METHOD_NAME: &str = "GET";
        let total_amount = 1000;
        let http_request = HttpRequestMethodDTO::new(METHOD_NAME, total_amount);
        assert_eq!(http_request, HttpRequestMethodDTO::decode(&http_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const METHOD_NAME: &str = "GET";
        let total_amount = 1000;
        let http_request = HttpRequestMethodDTO::new(METHOD_NAME, total_amount);
        assert_eq!(http_request.get_type(), HttpRequestMethodDTO::get_data_type());
    }
}