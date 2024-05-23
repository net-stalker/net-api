use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "buffered-network-packets-request";

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct BufferedNetworkPacketsRequestDTO {}
impl API for BufferedNetworkPacketsRequestDTO { }

impl BufferedNetworkPacketsRequestDTO {}

impl Encoder for BufferedNetworkPacketsRequestDTO {
    fn encode(&self) -> Vec<u8> {
        vec![]
    }
}

impl Decoder for BufferedNetworkPacketsRequestDTO {
    fn decode(_data: &[u8]) -> Self {
        Self::default()
    }
}

impl Typed for BufferedNetworkPacketsRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}


#[cfg(test)]
mod tests {
    use net_core_api::core::typed_api::Typed;
    use crate::api::network_packet::buffered_network_packets_request::BufferedNetworkPacketsRequestDTO;

    #[test]
    fn test_getting_data_types() {
        let buffered_network_packets_request = BufferedNetworkPacketsRequestDTO::default(); 
        assert_eq!(buffered_network_packets_request.get_type(), BufferedNetworkPacketsRequestDTO::get_data_type());
        assert_eq!(buffered_network_packets_request.get_type(), super::DATA_TYPE);
    }
}
