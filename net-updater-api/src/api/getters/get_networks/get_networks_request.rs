use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "get-networks-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GetNetworksRequestDTO { }
impl API for GetNetworksRequestDTO { }

impl GetNetworksRequestDTO {
    pub fn new() -> Self {
        GetNetworksRequestDTO::default()
    }
}

impl Encoder for GetNetworksRequestDTO {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl Decoder for GetNetworksRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> GetNetworksRequestDTO {
        GetNetworksRequestDTO::new()
    }
}

impl Typed for GetNetworksRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}