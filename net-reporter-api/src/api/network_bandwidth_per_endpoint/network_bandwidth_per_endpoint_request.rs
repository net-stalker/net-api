use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;


const DATA_TYPE: &str = "bandwidth-per-endpoint-request";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkBandwidthPerEndpointRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    // may be expandable in future
    // factors can be added here to make it possible to add charts
    // to dashboard with filters at once
    // TODO: add filters here
    // TODO: add filtering factor (a string)
}
impl API for NetworkBandwidthPerEndpointRequestDTO { }

impl NetworkBandwidthPerEndpointRequestDTO {
    pub fn new(start_date_time: i64, end_date_time: i64) -> Self {
        NetworkBandwidthPerEndpointRequestDTO {
            start_date_time,
            end_date_time,
        }
    }
    pub fn get_start_date_time(&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time(&self) -> i64 {
        self.end_date_time
    }
}

impl Encoder for NetworkBandwidthPerEndpointRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("start_date_time");
        writer.write_i64(self.start_date_time).unwrap();

        writer.set_field_name("end_date_time");
        writer.write_i64(self.end_date_time).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkBandwidthPerEndpointRequestDTO {
    fn decode(data: &[u8]) -> Self where Self: Sized {
        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        NetworkBandwidthPerEndpointRequestDTO::new(
            start_date_time,
            end_date_time
        )
    }
}

impl Typed for NetworkBandwidthPerEndpointRequestDTO {
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

    use net_core_api::encoder_api::Encoder;
    use net_core_api::decoder_api::Decoder;
    use net_core_api::typed_api::Typed;

    use crate::api::network_bandwidth_per_endpoint::network_bandwidth_per_endpoint_request::NetworkBandwidthPerEndpointRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_bandwidth_per_endpoint_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let bandwidth_per_endpoint_request = NetworkBandwidthPerEndpointRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
        );

        let mut binary_user_reader = ReaderBuilder::new().build(bandwidth_per_endpoint_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());
    }

    #[test]
    fn endec_bandwidth_per_endpoint_request() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let bandwidth_per_endpoint_request = NetworkBandwidthPerEndpointRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
        );
        assert_eq!(bandwidth_per_endpoint_request, NetworkBandwidthPerEndpointRequestDTO::decode(&bandwidth_per_endpoint_request.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;

        let bandwidth_per_endpoint_request = NetworkBandwidthPerEndpointRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
        );
        assert_eq!(bandwidth_per_endpoint_request.get_type(), NetworkBandwidthPerEndpointRequestDTO::get_data_type());
        assert_eq!(bandwidth_per_endpoint_request.get_type(), super::DATA_TYPE);
    }
}