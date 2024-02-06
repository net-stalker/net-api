use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::element::reader::ElementReader;

use net_core_api::api::API;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;


const DATA_TYPE: &str = "graph_edge";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphEdgeDTO {
    src_id: String,
    dst_id: String,
    communication_types: Vec<String>,
}
impl API for GraphEdgeDTO { }

impl GraphEdgeDTO {
    pub fn new(src_id: &str, dst_id: &str, communication_types: &[String]) -> Self {
        GraphEdgeDTO {
            src_id: src_id.into(), 
            dst_id: dst_id.into(),
            communication_types: communication_types.to_vec(),
        }
    }

    pub fn get_src_id(&self) -> &str {
        &self.src_id
    }

    pub fn get_dst_id(&self) -> &str {
        &self.dst_id
    }

    pub fn get_communication_types(&self) -> &[String] {
        self.communication_types.as_slice()
    }
}

impl Encoder for GraphEdgeDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("src_id");
        writer.write_string(&self.src_id).unwrap();

        writer.set_field_name("dst_id");
        writer.write_string(&self.dst_id).unwrap();

        writer.set_field_name("communication_types");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for communication_type in &self.communication_types {
            writer.write_string(communication_type).unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for GraphEdgeDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let src_id = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let dst_id = binding.text();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut communication_types = Vec::<String>::with_capacity(elements.len());
        for element in elements {
            communication_types.push(element.as_text().unwrap().to_string());
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        GraphEdgeDTO::new(
            src_id,
            dst_id,
            communication_types.as_slice(),
        )
    }
}

impl Typed for GraphEdgeDTO {
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

    use crate::api::network_graph::graph_edge::GraphEdgeDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_edge() {
        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let communication_types: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];
        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID, communication_types.as_slice());
        let mut binary_user_reader = ReaderBuilder::new().build(graph_edge.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("src_id", binary_user_reader.field_name().unwrap());
        assert_eq!(SRC_ID,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("dst_id", binary_user_reader.field_name().unwrap());
        assert_eq!(DST_ID,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("communication_types", binary_user_reader.field_name().unwrap());

        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!(communication_types.get(0).unwrap(), binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!(communication_types.get(1).unwrap(), binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!(communication_types.get(2).unwrap(), binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_graph_edge() {
        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let factors: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];
        let graph_edge = GraphEdgeDTO::new(SRC_ID, DST_ID, factors.as_slice());
        assert_eq!(graph_edge, GraphEdgeDTO::decode(&graph_edge.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let factors: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];
        let graph_edge = GraphEdgeDTO::new(SRC_ID, DST_ID, factors.as_slice());
        assert_eq!(graph_edge.get_type(), GraphEdgeDTO::get_data_type());
        assert_eq!(graph_edge.get_type(), super::DATA_TYPE);
    }
}