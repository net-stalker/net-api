use ion_rs;
use ion_rs::{IonType, IonWriter};
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DashboardRequestDTO {
    chart_requests: Vec<Envelope>,
}

impl DashboardRequestDTO {
    pub fn new (chart_requests: &[Envelope]) -> Self {
        Self { chart_requests: chart_requests.to_vec() }
    }

    pub fn _new (chart_requests: Vec<Envelope>) -> Self {
        Self { chart_requests }
    }

    pub fn get_network_graph_request(&self) -> &[Envelope] {
        &self.chart_requests
    }
}


impl Encoder for DashboardRequestDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
            let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
            let text_writer_builder = ion_rs::TextWriterBuilder::new(TextKind::Compact);

        #[cfg(feature = "ion-binary")]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        #[cfg(feature = "ion-text")]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("chart_requests");
        writer.step_in(IonType::List).unwrap();

        self.chart_requests.iter().for_each(|chart_request| {
            writer.step_in(IonType::Struct).unwrap();
            writer.set_field_name("group_id");
            match chart_request.get_group_id() {
                Ok(id) => writer.write_string(id).unwrap(),
                Err(_) => writer.write_null(IonType::String).unwrap(),
            };

            writer.set_field_name("agent_id");
            match chart_request.get_agent_id() {
                Ok(id) => writer.write_string(id).unwrap(),
                Err(_) => writer.write_null(IonType::String).unwrap(),
            };

            writer.set_field_name("type");
            writer.write_string(chart_request.get_type()).unwrap();

            writer.set_field_name("data");
            writer.write_blob(chart_request.get_data()).unwrap();

            writer.step_out().unwrap();
        });
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for DashboardRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        let chart_requests_raw = binary_user_reader.read_all_elements().unwrap();
        let mut chart_requests: Vec<Envelope> = Vec::with_capacity(chart_requests_raw.len());

        chart_requests_raw.iter().for_each(|element| {
            let raw_structure = element.as_struct().unwrap();
            let agent_id = raw_structure.get("agent_id").unwrap().as_string();
            let group_id = raw_structure.get("group_id").unwrap().as_string();
            let ty = raw_structure.get("type").unwrap().as_string().unwrap();
            let data = raw_structure.get("data").unwrap().as_blob().unwrap();
            chart_requests.push(Envelope::new(group_id, agent_id, ty, data));
        });
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
        DashboardRequestDTO::_new(chart_requests)
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

    use crate::api::dashboard_request::DashboardRequestDTO;

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

        let binding = binary_user_reader.read_all_elements().unwrap();

        let raw_element_1 = binding[0].as_struct().unwrap();
        assert_eq!(raw_element_1.get("group_id").unwrap().as_string(), group_id);
        assert_eq!(raw_element_1.get("agent_id").unwrap().as_string(), None);
        assert_eq!(raw_element_1.get("type").unwrap().as_string().unwrap(), TYPE1);
        assert_eq!(raw_element_1.get("data").unwrap().as_blob().unwrap(), data1);


        let raw_element_2 = binding[1].as_struct().unwrap();
        assert_eq!(raw_element_2.get("group_id").unwrap().as_string(), None);
        assert_eq!(raw_element_2.get("agent_id").unwrap().as_string(), agent_id);
        assert_eq!(raw_element_2.get("type").unwrap().as_string().unwrap(), TYPE2);
        assert_eq!(raw_element_2.get("data").unwrap().as_blob().unwrap(), data2);

        let raw_element_3 = binding[2].as_struct().unwrap();
        assert_eq!(raw_element_3.get("group_id").unwrap().as_string(), None);
        assert_eq!(raw_element_3.get("agent_id").unwrap().as_string(), None);
        assert_eq!(raw_element_3.get("type").unwrap().as_string().unwrap(), TYPE3);
        assert_eq!(raw_element_3.get("data").unwrap().as_blob().unwrap(), data3);

        let raw_element_4 = binding[3].as_struct().unwrap();
        assert_eq!(raw_element_4.get("group_id").unwrap().as_string(), group_id);
        assert_eq!(raw_element_4.get("agent_id").unwrap().as_string(), agent_id);
        assert_eq!(raw_element_4.get("type").unwrap().as_string().unwrap(), TYPE4);
        assert_eq!(raw_element_4.get("data").unwrap().as_blob().unwrap(), data4);

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
}