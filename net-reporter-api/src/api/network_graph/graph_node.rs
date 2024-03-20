use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;
use ion_rs::ReaderBuilder;

use net_core_api::core::api::API;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;


const DATA_TYPE: &str = "graph_node";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphNodeDTO {
    node_id: String,
}
impl API for GraphNodeDTO { }

impl GraphNodeDTO {
    pub fn new(node_id: &str) -> Self {
        GraphNodeDTO { node_id: node_id.into() }
    }

    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }
}

impl Encoder for GraphNodeDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");
        
        writer.set_field_name("node_id");
        writer.write_string(&self.node_id).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for GraphNodeDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let node_id = binding.text();

        GraphNodeDTO::new(node_id)
    }
}

impl Typed for GraphNodeDTO {
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

    use net_core_api::core::encoder_api::Encoder;
    use net_core_api::core::decoder_api::Decoder;
    use net_core_api::core::typed_api::Typed;

    use crate::api::network_graph::graph_node::GraphNodeDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_node() {
        const NODE_ID: &str = "0.0.0.0:0000";

        let graph_node = GraphNodeDTO::new(NODE_ID);
        let mut binary_user_reader = ReaderBuilder::new().build(graph_node.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("node_id", binary_user_reader.field_name().unwrap());
        assert_eq!(NODE_ID, binary_user_reader.read_string().unwrap().text());
    }

    #[test]
    fn endec_graph_node() {
        const NODE_ID: &str = "0.0.0.0:0000";

        let graph_node = GraphNodeDTO::new(NODE_ID);
        assert_eq!(graph_node, GraphNodeDTO::decode(&graph_node.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NODE_ID: &str = "0.0.0.0:0000";

        let graph_node = GraphNodeDTO::new(NODE_ID);
        assert_eq!(graph_node.get_type(), GraphNodeDTO::get_data_type());
        assert_eq!(graph_node.get_type(), super::DATA_TYPE);
    }
}