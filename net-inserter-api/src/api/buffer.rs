use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "flush-buffer-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FlushBufferRequestDTO { }
impl API for FlushBufferRequestDTO { }

impl FlushBufferRequestDTO {
    pub fn new() -> Self {
        FlushBufferRequestDTO::default()
    }
}

impl Encoder for FlushBufferRequestDTO {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl Decoder for FlushBufferRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> FlushBufferRequestDTO {
        FlushBufferRequestDTO::new()
    }
}

impl Typed for FlushBufferRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}
