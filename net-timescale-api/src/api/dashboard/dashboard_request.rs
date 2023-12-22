use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_proto_api::api::API;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;


const DATA_TYPE: &str = "dashboard_request";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DashboardRequestDTO {
    chart_requests: Vec<Envelope>,
}
impl API for DashboardRequestDTO { }

impl DashboardRequestDTO {
    pub fn new (chart_requests: &[Envelope]) -> Self {
        Self { chart_requests: chart_requests.to_vec() }
    }

    fn m_new(chart_requests: Vec<Envelope>) -> Self {
        Self { chart_requests }
    }

    pub fn get_chart_requests(&self) -> &[Envelope] {
        &self.chart_requests
    }

    pub fn get_type() -> &'static str {
        DATA_TYPE
    }
}

impl Encoder for DashboardRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();       
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("chart_requests");
        writer.step_in(IonType::List).unwrap();

        self.chart_requests.iter().for_each(|chart_request| {
            writer.write_blob(chart_request.encode()).unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for DashboardRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        let chart_requests_raw = binary_user_reader.read_all_elements().unwrap();
        let mut chart_requests: Vec<Envelope> = Vec::with_capacity(chart_requests_raw.len());

        chart_requests_raw.iter().for_each(|element| {
            chart_requests.push(Envelope::decode(element.as_blob().unwrap()));
        });
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
        DashboardRequestDTO::m_new(chart_requests)
    }
}

impl Typed for DashboardRequestDTO {
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

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::envelope::envelope::Envelope;
    use net_proto_api::typed_api::Typed;

    use crate::api::dashboard::dashboard_request::DashboardRequestDTO;

    #[test]
    fn reader_correctly_read_encoded_ng_request() {
        let group_id: Option<&str> = Some("some-group-id");
        let agent_id: Option<&str> = Some("some-agent-id");

        const TYPE1: &str = "type1";
        const TYPE2: &str = "type2";
        const TYPE3: &str = "type3";
        const TYPE4: &str = "type4";

        let data1: Vec<u8> = vec![1,2,3];
        let data2: Vec<u8> = vec![4,5,6];
        let data3: Vec<u8> = vec![7,8,9];
        let data4: Vec<u8> = vec![10,11,12];

        let chart_requests: Vec<Envelope> = vec![
            Envelope::new(group_id, None, TYPE1, data1.as_slice()),
            Envelope::new(None, agent_id, TYPE2, data2.as_slice()),
            Envelope::new(None, None, TYPE3, data3.as_slice()),
            Envelope::new(group_id, agent_id, TYPE4, data4.as_slice()),
        ];

        let dashboard_request = DashboardRequestDTO::new(chart_requests.as_slice());

        let mut binary_user_reader = ReaderBuilder::new().build(dashboard_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("chart_requests", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();

        let encoded_chart_requests: Vec<Envelope> = binary_user_reader.read_all_elements().unwrap().iter().map(|element| {
            Envelope::decode(element.as_blob().unwrap())
        }).collect();

        assert_eq!(encoded_chart_requests, chart_requests);

        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_ng_request() {
        let group_id: Option<&str> = Some("some-group-id");
        let agent_id: Option<&str> = Some("some-agent-id");

        const TYPE1: &str = "type1";
        const TYPE2: &str = "type2";
        const TYPE3: &str = "type3";
        const TYPE4: &str = "type4";

        let data1: Vec<u8> = vec![1,2,3];
        let data2: Vec<u8> = vec![4,5,6];
        let data3: Vec<u8> = vec![7,8,9];
        let data4: Vec<u8> = vec![10,11,12];

        let chart_requests: Vec<Envelope> = vec![
            Envelope::new(group_id, None, TYPE1, data1.as_slice()),
            Envelope::new(None, agent_id, TYPE2, data2.as_slice()),
            Envelope::new(None, None, TYPE3, data3.as_slice()),
            Envelope::new(group_id, agent_id, TYPE4, data4.as_slice()),
        ];

        let dashboard_request = DashboardRequestDTO::new(chart_requests.as_slice());
        assert_eq!(dashboard_request, DashboardRequestDTO::decode(&dashboard_request.encode()));
    }
    #[test]
    fn test_getting_data_types() {
        let group_id: Option<&str> = Some("some-group-id");
        let agent_id: Option<&str> = Some("some-agent-id");

        const TYPE1: &str = "type1";
        const TYPE2: &str = "type2";
        const TYPE3: &str = "type3";
        const TYPE4: &str = "type4";

        let data1: Vec<u8> = vec![1,2,3];
        let data2: Vec<u8> = vec![4,5,6];
        let data3: Vec<u8> = vec![7,8,9];
        let data4: Vec<u8> = vec![10,11,12];

        let chart_requests: Vec<Envelope> = vec![
            Envelope::new(group_id, None, TYPE1, data1.as_slice()),
            Envelope::new(None, agent_id, TYPE2, data2.as_slice()),
            Envelope::new(None, None, TYPE3, data3.as_slice()),
            Envelope::new(group_id, agent_id, TYPE4, data4.as_slice()),
        ];

        let dashboard_request = DashboardRequestDTO::new(chart_requests.as_slice());
        assert_eq!(dashboard_request.get_type(), DashboardRequestDTO::get_data_type());
        assert_eq!(dashboard_request.get_type(), super::DATA_TYPE);
    }
}