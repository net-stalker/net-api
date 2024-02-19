use ion_rs;

use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;


const DATA_TYPE: &str = "network_bandwidth_filters";

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkBandwidthFiltersDTO {
    protocols: Vec<String>,
    include_protocols_mode: bool,
    endpoints: Vec<String>,
    include_endpoints_mode: bool, 
}
impl API for NetworkBandwidthFiltersDTO { }

impl NetworkBandwidthFiltersDTO {
    pub fn new(protocols: &[String], include_protocols_mode: bool, endpoints: &[String], include_endpoints_mode: bool) -> Self {
        NetworkBandwidthFiltersDTO {
            protocols: protocols.to_vec(),
            include_protocols_mode,
            endpoints: endpoints.to_vec(),
            include_endpoints_mode,
        }
    }

    pub fn get_protocols(&self) -> &[String] {
        self.protocols.as_slice()
    }

    pub fn is_include_protocols_mode(&self) -> bool {
        self.include_protocols_mode
    }

    pub fn get_endpoints(&self) -> &[String] {
        self.endpoints.as_slice()
    }

    pub fn is_include_endpints_mode(&self) -> bool {
        self.include_endpoints_mode
    }
}

impl Encoder for NetworkBandwidthFiltersDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("protocols");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.protocols.iter().for_each(|protocol| {
            writer.write_string(protocol).unwrap();
        });
        writer.step_out().unwrap();

        writer.set_field_name("include_protocols_mode");
        writer.write_bool(self.include_protocols_mode).unwrap();

        writer.set_field_name("endpoints");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        self.endpoints.iter().for_each(|endpoint| {
            writer.write_string(endpoint).unwrap();
        });
        writer.step_out().unwrap();

        writer.set_field_name("include_endpoints_mode");
        writer.write_bool(self.include_endpoints_mode).unwrap();
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthFiltersDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let protocols_elements = binary_user_reader.read_all_elements().unwrap();
        let mut protocols = Vec::with_capacity(protocols_elements.len());
        protocols_elements.iter().for_each(|element| {
            let protocol = element.as_string().unwrap();
            protocols.push(protocol.to_owned());
        });

        binary_user_reader.step_out().unwrap();
        
        binary_user_reader.next().unwrap();
        let include_protocols = binary_user_reader.read_bool().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let endpoints_elements = binary_user_reader.read_all_elements().unwrap();
        let mut endpoints = Vec::with_capacity(endpoints_elements.len());
        endpoints_elements.iter().for_each(|element| {
            let endpoint = element.as_string().unwrap();
            endpoints.push(endpoint.to_owned());
        });
        
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        let include_endpoints = binary_user_reader.read_bool().unwrap();

        NetworkBandwidthFiltersDTO::new(
            protocols.as_slice(),
            include_protocols,
            endpoints.as_slice(),
            include_endpoints
        )
    }
}

impl Typed for NetworkBandwidthFiltersDTO {
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

    use crate::api::network_bandwidth::network_bandwidth_filters::NetworkBandwidthFiltersDTO;
    
    #[test]
    fn reader_correctly_read_encoded_ng_request() {
        let protocols = vec!["TCP".to_string(), "UDP".to_string()];
        const INCLUDE_PROTOCOLS_MODE: bool = false;
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;

        let network_bandwidth_filters = NetworkBandwidthFiltersDTO::new(
            &protocols,
            INCLUDE_PROTOCOLS_MODE,
            &endpoints,
            INCLUDE_ENDPOINTS_MODE
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_filters.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("protocols", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), protocols.len());
        for (element, core_protocol) in elements.iter().zip(protocols.as_slice()) {
            let protocol = element.as_string().unwrap();
            assert_eq!(protocol, *core_protocol);
        }
        binary_user_reader.step_out().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_protocols_mode", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE_PROTOCOLS_MODE,  binary_user_reader.read_bool().unwrap());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("endpoints", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), protocols.len());
        for (element, core_endpoint) in elements.iter().zip(endpoints.as_slice()) {
            let endpoint = element.as_string().unwrap();
            assert_eq!(endpoint, *core_endpoint);
        }
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_endpoints_mode", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE_ENDPOINTS_MODE,  binary_user_reader.read_bool().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_ng_request() {
        let protocols = vec!["TCP".to_string(), "UDP".to_string()];
        const INCLUDE_PROTOCOLS_MODE: bool = false;
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;

        let network_bandwidth_filters = NetworkBandwidthFiltersDTO::new(
            &protocols,
            INCLUDE_PROTOCOLS_MODE,
            &endpoints,
            INCLUDE_ENDPOINTS_MODE
        );
        assert_eq!(network_bandwidth_filters, NetworkBandwidthFiltersDTO::decode(&network_bandwidth_filters.encode()));
    }
}