use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

const DATA_TYPE: &str = "http_client";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponseDTO {
    bucket_timestamp: i64,
    client: String,
    server: String,
    response: i64,
}
impl API for HttpResponseDTO { }

impl HttpResponseDTO {
    pub fn new(bucket_timestamp: i64, client: &str, server: &str, response: i64) -> Self {
        HttpResponseDTO {
            bucket_timestamp,
            client: client.to_string(),
            server: server.to_string(),
            response,
        }
    }

    pub fn get_bucket_timestamp(&self) -> i64 {
        self.bucket_timestamp
    }

    pub fn get_client(&self) -> &str {
        &self.client
    }

    pub fn get_server(&self) -> &str {
        &self.server
    }

    pub fn get_response(&self) -> i64 {
        self.response
    }
}

impl Encoder for HttpResponseDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("bucket_timestamp");
        writer.write_i64(self.bucket_timestamp).unwrap();

        writer.set_field_name("client");
        writer.write_string(&self.client).unwrap();

        writer.set_field_name("server");
        writer.write_string(&self.server).unwrap();

        writer.set_field_name("response");
        writer.write_i64(self.response).unwrap();
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpResponseDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let bucket_timestamp = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let client = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let server = binding.text();

        binary_user_reader.next().unwrap();
        let response = binary_user_reader.read_i64().unwrap();

        binary_user_reader.step_out().unwrap();

        HttpResponseDTO::new(bucket_timestamp, client, server, response)
    }
}

impl Typed for HttpResponseDTO {
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

    use crate::api::http_responses::http_response::HttpResponseDTO;

    #[test]
    fn reader_correctly_read_encoded_http_response() {
        const BUCKET_TIMESTAMP: i64 = 123456789;
        const CLIENT: &str = "0.0.0.0";
        const SERVER: &str = "1.1.1.1";
        const RESPONSE: i64 = 200;
        let http_response = HttpResponseDTO::new(BUCKET_TIMESTAMP, CLIENT, SERVER, RESPONSE);
        let mut binary_user_reader = ReaderBuilder::new().build(http_response.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bucket_timestamp", binary_user_reader.field_name().unwrap());
        assert_eq!(BUCKET_TIMESTAMP, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("client", binary_user_reader.field_name().unwrap());
        assert_eq!(CLIENT,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("server", binary_user_reader.field_name().unwrap());
        assert_eq!(SERVER,  binary_user_reader.read_string().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("response", binary_user_reader.field_name().unwrap());
        assert_eq!(RESPONSE,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_response() {
        const BUCKET_TIMESTAMP: i64 = 123456789;
        const CLIENT: &str = "0.0.0.0";
        const SERVER: &str = "1.1.1.1";
        const RESPONSE: i64 = 200;
        let http_response = HttpResponseDTO::new(BUCKET_TIMESTAMP, CLIENT, SERVER, RESPONSE);
        assert_eq!(http_response, HttpResponseDTO::decode(&http_response.encode()));
    }
}