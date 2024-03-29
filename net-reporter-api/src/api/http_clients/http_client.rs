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

const DATA_TYPE: &str = "http_client";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpClientDTO {
    endpoint: String,
    user_agent: Option<String>,
    requests_amount: i64,
}
impl API for HttpClientDTO { }

impl HttpClientDTO {
    pub fn new(endpoint: &str, user_agent: Option<&str>, requests_amount: i64) -> Self {
        HttpClientDTO {
            endpoint: endpoint.to_string(),
            user_agent: user_agent.map(|user_agent| user_agent.to_string()),
            requests_amount,
        }
    }

    pub fn get_endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn get_user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    pub fn get_requests_amount(&self) -> i64 {
        self.requests_amount
    }
}

impl Encoder for HttpClientDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("endpoint");
        writer.write_string(&self.endpoint).unwrap();

        writer.set_field_name("user_agent");
        match &self.user_agent {
            Some(user_agent) => writer.write_string(user_agent).unwrap(),
            None => writer.write_null(IonType::String).unwrap(),
        }

        writer.set_field_name("requests_amount");
        writer.write_i64(self.requests_amount).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpClientDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let endpoint = binding.text();

        binary_user_reader.next().unwrap();

        let user_agent = match binary_user_reader.current() {
            StreamItem::Value(_) => {
                let binding = binary_user_reader.read_string().unwrap();
                Some(binding.text().to_string())
            },
            _ => None,
        };

        binary_user_reader.next().unwrap();
        let requests_amount = binary_user_reader.read_i64().unwrap();

        binary_user_reader.step_out().unwrap();

        HttpClientDTO::new(endpoint, user_agent.as_deref(), requests_amount)
    }
}

impl Typed for HttpClientDTO {
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

    use crate::api::http_clients::http_client::HttpClientDTO;

    #[test]
    fn reader_correctly_read_encoded_http_client() {
        const ENDPOINT: &str = "0.0.0.0";
        const USER_AGENT: &str = "Mozilla/5.0";
        const REQUESTS_AMOUNT: i64 = 123123;
        let http_client = HttpClientDTO::new(ENDPOINT, Some(USER_AGENT), REQUESTS_AMOUNT);
        let mut binary_user_reader = ReaderBuilder::new().build(http_client.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("endpoint", binary_user_reader.field_name().unwrap());
        assert_eq!(ENDPOINT,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("user_agent", binary_user_reader.field_name().unwrap());
        assert_eq!(USER_AGENT,  binary_user_reader.read_string().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("requests_amount", binary_user_reader.field_name().unwrap());
        assert_eq!(REQUESTS_AMOUNT,  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_client() {
        const ENDPOINT: &str = "0.0.0.0";
        const USER_AGENT: &str = "Mozilla/5.0";
        const REQUESTS_AMOUNT: i64 = 123123;
        let http_client = HttpClientDTO::new(ENDPOINT, Some(USER_AGENT), REQUESTS_AMOUNT);
        assert_eq!(http_client, HttpClientDTO::decode(&http_client.encode()));
    }
}
