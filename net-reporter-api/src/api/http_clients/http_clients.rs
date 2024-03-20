use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use super::http_client::HttpClientDTO;


const DATA_TYPE: &str = "http_clients";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpClientsDTO {
    http_clients: Vec<HttpClientDTO>,
}
impl API for HttpClientsDTO { }

impl HttpClientsDTO {
    pub fn new(http_clients: &[HttpClientDTO],) -> Self {
        HttpClientsDTO {
            http_clients: http_clients.to_vec(),
        }
    }

    pub fn get_http_clients(&self) -> &[HttpClientDTO] {
        &self.http_clients
    }
}

impl Encoder for HttpClientsDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("http_clients");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.http_clients.iter().for_each(|http_clients| {
            let data = http_clients.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpClientsDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let http_clients_elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_clients = Vec::with_capacity(http_clients_elements.len());
        http_clients_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let bucket = HttpClientDTO::decode(data);
            http_clients.push(bucket);
        });

        Self::new(&http_clients)
    }
}

impl Typed for HttpClientsDTO {
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
    use ion_rs::element::reader::ElementReader;

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;

    use crate::api::http_clients::http_client::HttpClientDTO;
    use crate::api::http_clients::http_clients::HttpClientsDTO;
    


    #[test]
    fn reader_correctly_read_total_http_clients() {
        const ENDPOINT: &str = "0.0.0.0";
        const USER_AGENT: &str = "Mozilla/5.0";
        const REQUESTS_AMOUNT: i64 = 123123;
        let http_client = HttpClientDTO::new(ENDPOINT, Some(USER_AGENT), REQUESTS_AMOUNT);
       
        let http_clients = vec![
            http_client.clone(),
            http_client.clone(),
            http_client.clone(),
        ];

        let http_clients_dto = HttpClientsDTO::new(&http_clients);

        let mut binary_user_reader = ReaderBuilder::new().build(http_clients_dto.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_clients", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), http_clients.len());
        for (element, http_client_core) in elements.iter().zip(http_clients.as_slice()) {
            let encoded_endpoint = HttpClientDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_endpoint, *http_client_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_total_http_clients() {
        const ENDPOINT: &str = "0.0.0.0";
        const USER_AGENT: &str = "Mozilla/5.0";
        const REQUESTS_AMOUNT: i64 = 123123;
        let http_client = HttpClientDTO::new(ENDPOINT, Some(USER_AGENT), REQUESTS_AMOUNT);
       
        let http_clients = vec![
            http_client.clone(),
            http_client.clone(),
            http_client.clone(),
        ];

        let http_clients_dto = HttpClientsDTO::new(&http_clients);

        assert_eq!(http_clients_dto, HttpClientsDTO::decode(&http_clients_dto.encode()));
    }
}