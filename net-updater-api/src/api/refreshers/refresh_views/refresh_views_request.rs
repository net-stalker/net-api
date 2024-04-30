use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "refresh-views-request";

#[derive(Debug, PartialEq, Eq)]
pub struct RefreshViewsRequestDTO { }
impl API for RefreshViewsRequestDTO { }

impl RefreshViewsRequestDTO {
    pub fn new() -> Self {
        RefreshViewsRequestDTO { }
    }
}

impl Encoder for RefreshViewsRequestDTO {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl Decoder for RefreshViewsRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> RefreshViewsRequestDTO {
        RefreshViewsRequestDTO::new()
    }
}

impl Typed for RefreshViewsRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}