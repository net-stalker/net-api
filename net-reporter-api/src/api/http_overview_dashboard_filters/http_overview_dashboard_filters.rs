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

use crate::api::network::networks::NetworksDTO;

const DATA_TYPE: &str = "http-overview-dashboard-filters";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpOverviewDashboardFiltersDTO {
    endpoints: Vec<String>,
    http_request_methods: Vec<String>,
    http_response_codes: Vec<String>,
    networks: NetworksDTO,
}
impl API for HttpOverviewDashboardFiltersDTO { }

impl HttpOverviewDashboardFiltersDTO {
    pub fn new(
        endpoints: &[String],
        http_request_methods: &[String],
        http_response_codes: &[String],
        networks: &NetworksDTO,
    ) -> Self {
        HttpOverviewDashboardFiltersDTO {
            endpoints: endpoints.to_vec(),
            http_request_methods: http_request_methods.to_vec(),
            http_response_codes: http_response_codes.to_vec(),
            networks: networks.clone(),
        }
    }

    pub fn get_endpoints(&self) -> &[String] {
        &self.endpoints
    }

    pub fn get_request_methods(&self) -> &[String] {
        &self.http_request_methods
    }

    pub fn get_response_codes(&self) -> &[String] {
        &self.http_response_codes
    }

    pub fn get_networks(&self) -> &NetworksDTO {
        &self.networks
    }
}

impl Encoder for HttpOverviewDashboardFiltersDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("endpoints");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for endpoint in &self.endpoints {
            writer.write_string(endpoint).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("http_request_methods");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for method in &self.http_request_methods {
            writer.write_string(method).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("http_response_codes");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for code in &self.http_response_codes {
            writer.write_string(code).unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("networks");
        writer.write_blob(&self.networks.encode()).unwrap();

        writer.step_out().unwrap();

        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for HttpOverviewDashboardFiltersDTO {
    fn decode(data: &[u8]) -> Self {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut endpoints = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            endpoints.push(element.as_text().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();
        
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_request_methods = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            http_request_methods.push(element.as_text().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut http_response_codes = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            http_response_codes.push(element.as_text().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        let networks = NetworksDTO::decode(binary_user_reader.read_blob().unwrap().as_slice());
        binary_user_reader.step_out().unwrap();

        HttpOverviewDashboardFiltersDTO::new(
            endpoints.as_slice(),
            http_request_methods.as_slice(),
            http_response_codes.as_slice(),
            &networks,
        )
    }
}

impl Typed for HttpOverviewDashboardFiltersDTO {
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
    use net_core_api::core::typed_api::Typed;

    use crate::api::http_overview_dashboard_filters::http_overview_dashboard_filters::HttpOverviewDashboardFiltersDTO;
    use crate::api::network::network::NetworkDTO;
    use crate::api::network::networks::NetworksDTO;

    fn get_networks() -> NetworksDTO {
        let networks = vec![
            NetworkDTO::new(
                "id1",
                "first name",
                "white"
            ),
            NetworkDTO::new(
                "id2",
                "second name",
                "black"
            ),
            NetworkDTO::new(
                "id3",
                "third name",
                "yellow"
            ),
        ];
        NetworksDTO::new(&networks)
    }

    #[test]
    fn reader_correctly_read_encoded_http_overview_filters() {
        let endpoints: Vec<String> = vec!["e1".to_string(), "e2".to_string(), "e3".to_string()];
        let requests: Vec<String> = vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()];
        let codes: Vec<String> = vec!["200".to_string(), "201".to_string(), "404".to_string()];
        let networks = get_networks(); 
        let filter_entry = HttpOverviewDashboardFiltersDTO::new(
            &endpoints,
            &requests,
            &codes,
            &networks,
        );
        let mut binary_user_reader = ReaderBuilder::new().build(filter_entry.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("endpoints", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let endeced_endpoints: Vec<String> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
          element.as_string().unwrap().to_owned()
        }).collect();

        assert_eq!(endpoints, endeced_endpoints);

        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_request_methods", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let endeced_requests: Vec<String> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
          element.as_string().unwrap().to_owned()
        }).collect();

        assert_eq!(requests, endeced_requests);

        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("http_response_codes", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let endeced_response_codes: Vec<String> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
          element.as_string().unwrap().to_owned()
        }).collect();

        assert_eq!(codes, endeced_response_codes);

        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Blob), binary_user_reader.next().unwrap());
        assert_eq!("networks", binary_user_reader.field_name().unwrap());
        assert_eq!(networks,  NetworksDTO::decode(binary_user_reader.read_blob().unwrap().as_slice()));

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_http_overview_filters() {
        let endpoints: Vec<String> = vec!["e1".to_string(), "e2".to_string(), "e3".to_string()];
        let requests: Vec<String> = vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()];
        let codes: Vec<String> = vec!["200".to_string(), "201".to_string(), "404".to_string()];
        let networks = get_networks(); 
        let filter_entry = HttpOverviewDashboardFiltersDTO::new(
            &endpoints,
            &requests,
            &codes,
            &networks,
        );
        assert_eq!(filter_entry, HttpOverviewDashboardFiltersDTO::decode(&filter_entry.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        let endpoints: Vec<String> = vec!["e1".to_string(), "e2".to_string(), "e3".to_string()];
        let requests: Vec<String> = vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()];
        let codes: Vec<String> = vec!["200".to_string(), "201".to_string(), "404".to_string()];
        let networks = get_networks(); 
        let filter_entry = HttpOverviewDashboardFiltersDTO::new(
            &endpoints,
            &requests,
            &codes,
            &networks,
        );
        assert_eq!(filter_entry.get_type(), HttpOverviewDashboardFiltersDTO::get_data_type());
        assert_eq!(filter_entry.get_type(), super::DATA_TYPE);
    }
}