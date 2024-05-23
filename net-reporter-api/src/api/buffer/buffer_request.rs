use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "buffer-request";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BufferRequestDTO { }
impl API for BufferRequestDTO { }

impl BufferRequestDTO {
    pub fn new() -> Self {
        BufferRequestDTO::default()
    }
}

impl Encoder for BufferRequestDTO {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl Decoder for BufferRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> BufferRequestDTO {
        BufferRequestDTO::new()
    }
}

impl Typed for BufferRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}
