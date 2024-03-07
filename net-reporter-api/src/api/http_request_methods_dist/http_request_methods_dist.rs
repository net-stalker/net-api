use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

use super::http_request::HttpRequestDTO;


const DATA_TYPE: &str = "http_request_methods_dist";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpRequestMethodsDistDTO {
    http_requests: Vec<HttpRequestDTO>,
}
impl API for HttpRequestMethodsDistDTO { }

impl HttpRequestMethodsDistDTO {
    pub fn new(http_requests: &[HttpRequestDTO]) -> Self {
        HttpRequestMethodsDistDTO {
            http_requests: http_requests.to_vec(),
        }
    }

    pub fn get_http_requests(&self) -> &[HttpRequestDTO] {
        &self.http_requests
    }
}

impl Encoder for HttpRequestMethodsDistDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("http_requests");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for http_request in &self.http_requests {
            writer.write_blob(http_request.encode()).unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpRequestMethodsDistDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_requests = Vec::<HttpRequestDTO>::with_capacity(elements.len());
        for element in elements {
            http_requests.push(HttpRequestDTO::decode(element.as_blob().unwrap()));
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        HttpRequestMethodsDistDTO::new(http_requests.as_slice())
    }
}

impl Typed for HttpRequestMethodsDistDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }
    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use ion_rs::element::reader::ElementReader;
    use ion_rs::IonType;
    use ion_rs::IonReader;
    use ion_rs::ReaderBuilder;
    use ion_rs::StreamItem;

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::typed_api::Typed;

    use crate::api::http_request_methods_dist::http_request::HttpRequestDTO;
    use crate::api::http_request_methods_dist::http_request_methods_dist::HttpRequestMethodsDistDTO;

    #[test]
    fn reader_correctly_read_encoded_request_methods_dist() {
        const GET_METHOD: &str = "GET";
        let get_total_amount = 1001;
        let get_http_request = HttpRequestDTO::new(GET_METHOD, get_total_amount);
        
        const POST_METHOD: &str = "POST";
        let post_total_amount = 1002;
        let post_http_request = HttpRequestDTO::new(POST_METHOD, post_total_amount);

        let dist = HttpRequestMethodsDistDTO::new([get_http_request, post_http_request].as_ref());
        let mut binary_user_reader = ReaderBuilder::new().build(dist.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_requests", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let endeced_http_requests: Vec<HttpRequestDTO> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
            HttpRequestDTO::decode(element.as_blob().unwrap())
        }).collect();

        assert_eq!(dist.get_http_requests(), endeced_http_requests);

        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_request_methods_dist() {
        const GET_METHOD: &str = "GET";
        let get_total_amount = 1001;
        let get_http_request = HttpRequestDTO::new(GET_METHOD, get_total_amount);
        
        const POST_METHOD: &str = "POST";
        let post_total_amount = 1002;
        let post_http_request = HttpRequestDTO::new(POST_METHOD, post_total_amount);

        let dist = HttpRequestMethodsDistDTO::new([get_http_request, post_http_request].as_ref());
        assert_eq!(dist, HttpRequestMethodsDistDTO::decode(&dist.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const GET_METHOD: &str = "GET";
        let get_total_amount = 1001;
        let get_http_request = HttpRequestDTO::new(GET_METHOD, get_total_amount);
        
        const POST_METHOD: &str = "POST";
        let post_total_amount = 1002;
        let post_http_request = HttpRequestDTO::new(POST_METHOD, post_total_amount);

        let dist = HttpRequestMethodsDistDTO::new([get_http_request, post_http_request].as_ref());
        assert_eq!(dist.get_type(), HttpRequestMethodsDistDTO::get_data_type());
        assert_eq!(dist.get_type(), super::DATA_TYPE);
    }
}