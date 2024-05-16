use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "clear-buffer-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ClearBufferRequestDTO { }
impl API for ClearBufferRequestDTO { }

impl ClearBufferRequestDTO {
    pub fn new() -> Self {
        ClearBufferRequestDTO::default()
    }
}

impl Encoder for ClearBufferRequestDTO {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl Decoder for ClearBufferRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> ClearBufferRequestDTO {
        ClearBufferRequestDTO::new()
    }
}

impl Typed for ClearBufferRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}
