use ion_rs;
use ion_rs::element::reader::ElementReader;
use ion_rs::{IonType, IonWriter};
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use crate::api::filters::Filters;


#[derive(Debug, PartialEq, Eq)]
pub struct NetworkGraphRequestDTO {
    start_date_time: i64,
    end_date_time: i64,
    subscribe: bool,
    filters: Option<Filters>,
}

impl NetworkGraphRequestDTO {
    pub fn new (start_date_time: i64, end_date_time: i64, subscribe: bool, filters: Option<Filters>) -> Self {
        NetworkGraphRequestDTO {
            start_date_time,
            end_date_time,
            subscribe,
            filters,
        }
    }

    pub fn get_start_date_time (&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time (&self) -> i64 {
        self.end_date_time
    }

    pub fn is_subscribe (&self) -> bool {
        self.subscribe
    }

    pub fn get_filters (&self) -> &Option<Filters> {
        &self.filters
    }
}

impl Encoder for NetworkGraphRequestDTO {
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

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("start_date_time");
        writer.write_i64(self.start_date_time).unwrap();

        writer.set_field_name("end_date_time");
        writer.write_i64(self.end_date_time).unwrap();

        writer.set_field_name("subscribe");
        writer.write_bool(self.subscribe).unwrap();

        writer.set_field_name("filters");
        match self.filters.as_ref() {
            Some(filters) => {
                writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

                writer.set_field_name("include");
                writer.write_bool(filters.is_include()).unwrap();

                writer.set_field_name("filters");
                writer.step_in(IonType::List).unwrap();
                filters.get_filters().iter().for_each(|filter| {
                    writer.write_string(filter).unwrap();
                });
                writer.step_out().unwrap();

                writer.step_out().unwrap();
            },
            None => writer.write_null(IonType::Struct).unwrap(),
        };

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkGraphRequestDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let start_date_time = binary_user_reader.read_i64().unwrap();
        
        binary_user_reader.next().unwrap();
        let end_date_time = binary_user_reader.read_i64().unwrap();

        binary_user_reader.next().unwrap();
        let subscribe = binary_user_reader.read_bool().unwrap();

        // let binding;
        binary_user_reader.next().unwrap();
        let filters = match binary_user_reader.current() {
            ion_rs::StreamItem::Value(_) => {
                binary_user_reader.step_in().unwrap();

                binary_user_reader.next().unwrap();
                let include = binary_user_reader.read_bool().unwrap();

                binary_user_reader.next().unwrap();
                binary_user_reader.step_in().unwrap();
                let elements = binary_user_reader.read_all_elements().unwrap();
                let mut filters = Vec::<String>::with_capacity(elements.len());
                elements.iter().for_each(|element| {
                    let filter = element.as_string().unwrap();
                    filters.push(filter.to_string());
                });
                // step out from the list
                binary_user_reader.step_out().unwrap();
                // step out from the root structure
                binary_user_reader.step_out().unwrap();
                Some(Filters::new(include, filters.as_slice()))
            },
            ion_rs::StreamItem::Null(_) => {
                None
            },
            //TODO: Return en error here in future
            ion_rs::StreamItem::Nothing => todo!(),
        };

        // step out from the root structure
        binary_user_reader.step_out().unwrap();

        NetworkGraphRequestDTO::new(
            start_date_time,
            end_date_time,
            subscribe,
            filters,
        )
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
    use crate::api::filters::Filters;

    use crate::api::network_graph_request::NetworkGraphRequestDTO;

    #[ignore]
    #[test]
    fn reader_correctly_read_encoded_ng_request() {
        todo!("finish this test");
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const SUBSCRIBE: bool = true;
        const INCLUDE: bool = true;

        let expected_filters: Vec<String> = ["f1", "f2", "f3"]
            .iter()
            .map(|filter| filter.to_string())
            .collect();

        let filters = Filters::new(INCLUDE, expected_filters.as_slice());


        let network_graph_request = NetworkGraphRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            SUBSCRIBE,
            Some(filters),
        );
        
        let mut binary_user_reader = ReaderBuilder::new().build(network_graph_request.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("start_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(START_DATE_TIME, binary_user_reader.read_i64().unwrap());
        
        assert_eq!(StreamItem::Value(IonType::Int), binary_user_reader.next().unwrap());
        assert_eq!("end_date_time", binary_user_reader.field_name().unwrap());
        assert_eq!(END_DATE_TIME,  binary_user_reader.read_i64().unwrap());

        assert_eq!(StreamItem::Value(IonType::Bool), binary_user_reader.next().unwrap());
        assert_eq!("subscribe", binary_user_reader.field_name().unwrap());
        assert_eq!(SUBSCRIBE,  binary_user_reader.read_bool().unwrap());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("filters", binary_user_reader.field_name().unwrap());
        binary_user_reader.step_in().unwrap();
        let filters: Vec<String> = binary_user_reader.read_all_elements().unwrap()
            .iter()
            .map(|element| element.as_string().unwrap().to_string())
            .collect();
        binary_user_reader.step_out().unwrap();
        assert_eq!(expected_filters, filters);
        binary_user_reader.step_out().unwrap();

    }

    #[test]
    fn endec_ng_request_filters_some() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const SUBSCRIBE: bool = true;

        const INCLUDE: bool = true;

        let expected_filters: Vec<String> = ["f1", "f2", "f3"]
            .iter()
            .map(|filter| filter.to_string())
            .collect();

        let filters = Filters::new(INCLUDE, expected_filters.as_slice());

        let network_graph_request = NetworkGraphRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            SUBSCRIBE,
            Some(filters),
        );
        assert_eq!(network_graph_request, NetworkGraphRequestDTO::decode(&network_graph_request.encode()));
    }

    #[test]
    fn endec_ng_request_filters_none() {
        const START_DATE_TIME: i64 = i64::MIN;
        const END_DATE_TIME: i64 = i64::MAX;
        const SUBSCRIBE: bool = true;

        let network_graph_request = NetworkGraphRequestDTO::new(
            START_DATE_TIME,
            END_DATE_TIME,
            SUBSCRIBE,
            None,
        );
        assert_eq!(network_graph_request, NetworkGraphRequestDTO::decode(&network_graph_request.encode()));
    }
}