use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

const DATA_TYPE: &str = "refresh-pcap-parsed-data-request";

#[derive(Debug, PartialEq, Eq)]
pub struct RefreshPcapParsedDataRequestDTO { }
impl API for RefreshPcapParsedDataRequestDTO { }

impl RefreshPcapParsedDataRequestDTO {
    pub fn new() -> Self {
        RefreshPcapParsedDataRequestDTO { }
    }
}

impl Encoder for RefreshPcapParsedDataRequestDTO {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl Decoder for RefreshPcapParsedDataRequestDTO {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> RefreshPcapParsedDataRequestDTO {
        RefreshPcapParsedDataRequestDTO::new()
    }
}

impl Typed for RefreshPcapParsedDataRequestDTO {
    fn get_data_type() -> &'static str {
        DATA_TYPE
    }

    fn get_type(&self) -> &str {
        Self::get_data_type()
    }
}