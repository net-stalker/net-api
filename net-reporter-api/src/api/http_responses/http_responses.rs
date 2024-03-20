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

use super::http_response::HttpResponseDTO;


const DATA_TYPE: &str = "http_responses";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponsesDTO {
    http_responses: Vec<HttpResponseDTO>,
}
impl API for HttpResponsesDTO { }

impl HttpResponsesDTO {
    pub fn new(http_responses: &[HttpResponseDTO],) -> Self {
        HttpResponsesDTO {
            http_responses: http_responses.to_vec(),
        }
    }

    pub fn get_http_responses(&self) -> &[HttpResponseDTO] {
        &self.http_responses
    }
}

impl Encoder for HttpResponsesDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("http_responses");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.http_responses.iter().for_each(|http_responses| {
            let data = http_responses.encode();
            writer.write_blob(data.as_slice()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpResponsesDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let http_responses_elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_responses = Vec::with_capacity(http_responses_elements.len());
        http_responses_elements.iter().for_each(|element| {
            let data = element.as_blob().unwrap();
            let http_response = HttpResponseDTO::decode(data);
            http_responses.push(http_response);
        });

        Self::new(&http_responses)
    }
}

impl Typed for HttpResponsesDTO {
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

    use crate::api::http_responses::http_response::HttpResponseDTO;
    use crate::api::http_responses::http_responses::HttpResponsesDTO;
    


    #[test]
    fn reader_correctly_read_total_http_responses() {
        const BUCKET_TIMESTAMP: i64 = 123456789;
        const CLIENT: &str = "0.0.0.0";
        const SERVER: &str = "1.1.1.1";
        const RESPONSE: i64 = 200;
        let http_response = HttpResponseDTO::new(BUCKET_TIMESTAMP, CLIENT, SERVER, RESPONSE);

        let http_responses = vec![
            http_response.clone(),
            http_response.clone(),
            http_response.clone(),
        ];

        let http_responses_dto = HttpResponsesDTO::new(&http_responses);

        let mut binary_user_reader = ReaderBuilder::new().build(http_responses_dto.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_responses", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), http_responses.len());
        for (element, http_responses_core) in elements.iter().zip(http_responses.as_slice()) {
            let encoded_http_response = HttpResponseDTO::decode(element.as_blob().unwrap());
            assert_eq!(encoded_http_response, *http_responses_core);
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_total_http_responses() {
        const BUCKET_TIMESTAMP: i64 = 123456789;
        const CLIENT: &str = "0.0.0.0";
        const SERVER: &str = "1.1.1.1";
        const RESPONSE: i64 = 200;
        let http_response = HttpResponseDTO::new(BUCKET_TIMESTAMP, CLIENT, SERVER, RESPONSE);

        let http_responses = vec![
            http_response.clone(),
            http_response.clone(),
            http_response.clone(),
        ];

        let http_responses_dto: HttpResponsesDTO = HttpResponsesDTO::new(&http_responses);


        assert_eq!(http_responses_dto, HttpResponsesDTO::decode(&http_responses_dto.encode()));
    }
}