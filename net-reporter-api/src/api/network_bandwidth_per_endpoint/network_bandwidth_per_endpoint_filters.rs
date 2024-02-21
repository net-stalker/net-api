use ion_rs;

use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use ion_rs::StreamItem;
use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

const DATA_TYPE: &str = "network_bandwidth_per_endpoint_filters";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkBandwidthPerEndpointFiltersDTO {
    protocols: Vec<String>,
    include_protocols_mode: Option<bool>,
    endpoints: Vec<String>,
    include_endpoints_mode: Option<bool>,
    bytes_lower_bound: Option<i64>,
    bytes_upper_bound: Option<i64>,
}
impl API for NetworkBandwidthPerEndpointFiltersDTO { }

impl NetworkBandwidthPerEndpointFiltersDTO {
    pub fn new(
        protocols: &[String],
        include_protocols_mode: Option<bool>,
        endpoints: &[String],
        include_endpoints_mode: Option<bool>,
        bytes_lower_bound: Option<i64>,
        bytes_upper_bound: Option<i64>,
    ) -> Self {
        NetworkBandwidthPerEndpointFiltersDTO {
            protocols: protocols.to_vec(),
            include_protocols_mode,
            endpoints: endpoints.to_vec(),
            include_endpoints_mode,
            bytes_lower_bound,
            bytes_upper_bound,
        }
    }

    pub fn get_protocols(&self) -> &[String] {
        self.protocols.as_slice()
    }

    pub fn is_include_protocols_mode(&self) -> Option<bool> {
        self.include_protocols_mode
    }

    pub fn get_endpoints(&self) -> &[String] {
        self.endpoints.as_slice()
    }

    pub fn is_include_endpoints_mode(&self) -> Option<bool> {
        self.include_endpoints_mode
    }

    pub fn get_bytes_lower_bound(&self) -> Option<i64> {
        self.bytes_lower_bound
    }

    pub fn get_bytes_upper_bound(&self) -> Option<i64> {
        self.bytes_upper_bound
    }
}

impl Encoder for NetworkBandwidthPerEndpointFiltersDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("include_protocols_mode");
        match self.include_protocols_mode {
            Some(include_protocols_mode) => {
                writer.write_bool(include_protocols_mode).unwrap();
                writer.set_field_name("protocols");
                writer.step_in(IonType::List).expect("Error while entering an ion list");
                self.protocols.iter().for_each(|protocol| {
                    writer.write_string(protocol).unwrap();
                });
                writer.step_out().unwrap();
            },
            None => writer.write_null(IonType::Bool).unwrap(),
        };

        writer.set_field_name("include_endpoints_mode");
        match self.include_endpoints_mode {
            Some(include_endpoints_mode) => {
                writer.write_bool(include_endpoints_mode).unwrap();
                writer.set_field_name("endpoints");
                writer.step_in(IonType::List).expect("Error while entering an ion list");
                self.endpoints.iter().for_each(|endpoint| {
                    writer.write_string(endpoint).unwrap();
                });
                writer.step_out().unwrap();
            },
            None => writer.write_null(IonType::Bool).unwrap(),
        }

        writer.set_field_name("bytes_lower_bound");
        match self.bytes_lower_bound {
            Some(bytes_lower_bound) => writer.write_i64(bytes_lower_bound).unwrap(),
            None => writer.write_null(IonType::Int).unwrap(),
        }

        writer.set_field_name("bytes_upper_bound");
        match self.bytes_upper_bound {
            Some(bytes_upper_bound) => writer.write_i64(bytes_upper_bound).unwrap(),
            None => writer.write_null(IonType::Int).unwrap(),
        }
        
        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthPerEndpointFiltersDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();

        let (include_protocols, protocols) = match binary_user_reader.current()  {
            StreamItem::Value(_) => {
                let include_protocols = binary_user_reader.read_bool().unwrap();
                binary_user_reader.next().unwrap();
                binary_user_reader.step_in().unwrap();
                let protocols_elements = binary_user_reader.read_all_elements().unwrap();
                let mut protocols = Vec::with_capacity(protocols_elements.len());
                protocols_elements.iter().for_each(|element| {
                    let protocol = element.as_string().unwrap();
                    protocols.push(protocol.to_owned());
                });

                binary_user_reader.step_out().unwrap();

                (Some(include_protocols), protocols)
            },
            _ => (None, vec![]),
        };

        binary_user_reader.next().unwrap();
        let (include_endpoints, endpoints) = match binary_user_reader.current() {
            StreamItem::Value(_) => {
                let include_endpoints = binary_user_reader.read_bool().unwrap();
                binary_user_reader.next().unwrap();
                binary_user_reader.step_in().unwrap();
                let endpoints_elements = binary_user_reader.read_all_elements().unwrap();
                let mut endpoints = Vec::with_capacity(endpoints_elements.len());
                endpoints_elements.iter().for_each(|element| {
                    let endpoint = element.as_string().unwrap();
                    endpoints.push(endpoint.to_owned());
                });
                
                binary_user_reader.step_out().unwrap();
                (Some(include_endpoints), endpoints)
            },
            _ => (None, vec![]),
        };

        binary_user_reader.next().unwrap();
        let bytes_lower_bound = match binary_user_reader.current() {
            StreamItem::Value(_) => Some(binary_user_reader.read_i64().unwrap()),
            _ => None,
        };

        binary_user_reader.next().unwrap();
        let bytes_upper_bound = match binary_user_reader.current() {
            StreamItem::Value(_) => Some(binary_user_reader.read_i64().unwrap()),
            _ => None,
        };

        NetworkBandwidthPerEndpointFiltersDTO::new(
            protocols.as_slice(),
            include_protocols,
            endpoints.as_slice(),
            include_endpoints,
            bytes_lower_bound,
            bytes_upper_bound,
        )
    }
}

impl Typed for NetworkBandwidthPerEndpointFiltersDTO {
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

    use crate::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_filters::NetworkBandwidthPerEndpointFiltersDTO;

    
    #[test]
    fn reader_correctly_read_encoded_nbpe_filters_0() {
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;
        let bytes_lower_bound = Some(100);
        
        let network_bandwidth_filters = NetworkBandwidthPerEndpointFiltersDTO::new(
            &[],
            None,
            &endpoints,
            Some(INCLUDE_ENDPOINTS_MODE),
            bytes_lower_bound,
            None,
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_filters.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Null(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_protocols_mode", binary_user_reader.field_name().unwrap());

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_endpoints_mode", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE_ENDPOINTS_MODE,  binary_user_reader.read_bool().unwrap());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("endpoints", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), endpoints.len());
        for (element, core_endpoint) in elements.iter().zip(endpoints.as_slice()) {
            let endpoint = element.as_string().unwrap();
            assert_eq!(endpoint, *core_endpoint);
        }
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_lower_bound", binary_user_reader.field_name().unwrap());
        assert_eq!(*bytes_lower_bound.as_ref().unwrap(),  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Null(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_upper_bound", binary_user_reader.field_name().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    
    #[test]
    fn reader_correctly_read_encoded_nbpe_filters_1() {
        let protocols = vec!["TCP".to_string(), "UDP".to_string()];
        const INCLUDE_PROTOCOLS_MODE: bool = false;
        let endpoints = vec![];
        let bytes_upper_bound = Some(100);
        
        let network_bandwidth_filters = NetworkBandwidthPerEndpointFiltersDTO::new(
            &protocols,
            Some(INCLUDE_PROTOCOLS_MODE),
            &endpoints,
            None,
            None,
            bytes_upper_bound,
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_filters.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_protocols_mode", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE_PROTOCOLS_MODE,  binary_user_reader.read_bool().unwrap());

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

        assert_eq!(StreamItem::Null(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_endpoints_mode", binary_user_reader.field_name().unwrap());

        assert_eq!(StreamItem::Null(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_lower_bound", binary_user_reader.field_name().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_upper_bound", binary_user_reader.field_name().unwrap());
        assert_eq!(*bytes_upper_bound.as_ref().unwrap(),  binary_user_reader.read_i64().unwrap());

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn reader_correctly_read_encoded_nbpe_filters_2() {
        let protocols = vec!["TCP".to_string(), "UDP".to_string()];
        const INCLUDE_PROTOCOLS_MODE: bool = false;
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;
        
        let network_bandwidth_filters = NetworkBandwidthPerEndpointFiltersDTO::new(
            &protocols,
            Some(INCLUDE_PROTOCOLS_MODE),
            &endpoints,
            Some(INCLUDE_ENDPOINTS_MODE),
            None,
            None,
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_bandwidth_filters.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("include_protocols_mode", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE_PROTOCOLS_MODE,  binary_user_reader.read_bool().unwrap());

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
        assert_eq!("include_endpoints_mode", binary_user_reader.field_name().unwrap());
        assert_eq!(INCLUDE_ENDPOINTS_MODE,  binary_user_reader.read_bool().unwrap());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("endpoints", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        assert_eq!(elements.len(), endpoints.len());
        for (element, core_endpoint) in elements.iter().zip(endpoints.as_slice()) {
            let endpoint = element.as_string().unwrap();
            assert_eq!(endpoint, *core_endpoint);
        }
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Null(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_lower_bound", binary_user_reader.field_name().unwrap());

        assert_eq!(StreamItem::Null(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("bytes_upper_bound", binary_user_reader.field_name().unwrap());


        binary_user_reader.step_out().unwrap();
    }


    #[test]
    fn endec_nbpe_filters() {
        let protocols = vec!["TCP".to_string(), "UDP".to_string()];
        const INCLUDE_PROTOCOLS_MODE: bool = false;
        let endpoints = vec!["0.0.0.0".to_string(), "1.1.1.1".to_string()];
        const INCLUDE_ENDPOINTS_MODE: bool = true;
        let bytes_lower_bound = Some(100);
        let bytes_upper_bound = Some(1000);

        let network_bandwidth_filters = NetworkBandwidthPerEndpointFiltersDTO::new(
            &protocols,
            Some(INCLUDE_PROTOCOLS_MODE),
            &endpoints,
            Some(INCLUDE_ENDPOINTS_MODE),
            bytes_lower_bound,
            bytes_upper_bound
        );
        assert_eq!(network_bandwidth_filters, NetworkBandwidthPerEndpointFiltersDTO::decode(&network_bandwidth_filters.encode()));
    }
}