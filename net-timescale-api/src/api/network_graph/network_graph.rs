use ion_rs;

use ion_rs::IonReader;
use ion_rs::IonType;
use ion_rs::IonWriter;

use ion_rs::ReaderBuilder;
use ion_rs::TextWriterBuilder;

use ion_rs::element::reader::ElementReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::api::API;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;

use super::graph_edge::GraphEdgeDTO;
use super::graph_node::GraphNodeDTO;


const DATA_TYPE: &str = "network_graph";

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkGraphDTO {
    graph_nodes: Vec<GraphNodeDTO>,
    graph_edges: Vec<GraphEdgeDTO>,
}
impl API for NetworkGraphDTO { }

impl NetworkGraphDTO {
    pub fn new(graph_nodes: &[GraphNodeDTO], graph_edges: &[GraphEdgeDTO]) -> Self {
        NetworkGraphDTO {
            graph_nodes: graph_nodes.to_vec(),
            graph_edges: graph_edges.to_vec(),
        }
    }

    pub fn get_graph_nodes(&self) -> &[GraphNodeDTO] {
        &self.graph_nodes
    }

    pub fn get_graph_edges(&self) -> &[GraphEdgeDTO] {
        &self.graph_edges
    }
}

impl Encoder for NetworkGraphDTO {
    fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        #[cfg(feature = "ion-binary")]
        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        #[cfg(feature = "ion-text")]
        let text_writer_builder = TextWriterBuilder::new(TextKind::Compact); 

        #[cfg(feature = "ion-binary")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();
        
        #[cfg(feature = "ion-text")]
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut writer = text_writer_builder.build(buffer).unwrap();

        writer.step_in(IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("graph_nodes");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for graph_node in &self.graph_nodes {
            writer.step_in(IonType::Struct).expect("Error while entering an ion struct");
            
            writer.set_field_name("node_id");
            writer.write_string(graph_node.get_node_id()).unwrap();
            
            writer.set_field_name("agent_id");
            writer.write_string(graph_node.get_agent_id()).unwrap();

            writer.step_out().unwrap();
        }
        writer.step_out().unwrap();

        writer.set_field_name("graph_edges");
        writer.step_in(IonType::List).expect("Error while entering an ion list");
        for graph_edge in &self.graph_edges {
            writer.step_in(IonType::Struct).expect("Error while entering an ion struct");
            
            writer.set_field_name("src_id");
            writer.write_string(graph_edge.get_src_id()).unwrap();
    
            writer.set_field_name("dst_id");
            writer.write_string(graph_edge.get_dst_id()).unwrap();

            writer.set_field_name("communication_types");
            writer.step_in(IonType::List).expect("Error while entering an ion list");
            for communication_type in graph_edge.get_communication_types() {
                writer.write_string(communication_type).unwrap();
            }
            writer.step_out().unwrap();
            
            writer.step_out().unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for NetworkGraphDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut graph_nodes = Vec::<GraphNodeDTO>::with_capacity(elements.capacity());
        for element in elements {
            let graph_node = element.as_struct().unwrap();
            let node_id = graph_node.get("node_id").unwrap().as_text().unwrap();
            let agent_id = graph_node.get("agent_id").unwrap().as_text().unwrap();
            graph_nodes.push(GraphNodeDTO::new(node_id, agent_id));
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();
        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut graph_edges = Vec::<GraphEdgeDTO>::with_capacity(elements.capacity());
        for element in elements {
            let graph_edge = element.as_struct().unwrap();
            let src_id = graph_edge.get("src_id").unwrap().as_text().unwrap();
            let dst_id = graph_edge.get("dst_id").unwrap().as_text().unwrap();
            let ct = graph_edge.get("communication_types").unwrap().as_sequence().unwrap();
            let mut communication_types = Vec::<String>::with_capacity(ct.len());
            ct.elements().for_each(|el| communication_types.push(el.as_text().unwrap().to_string()));
            graph_edges.push(GraphEdgeDTO::new(src_id, dst_id, communication_types.as_slice()));
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        NetworkGraphDTO {
            graph_nodes,
            graph_edges,
        }
    }
}

impl Typed for NetworkGraphDTO {
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

    use net_proto_api::decoder_api::Decoder;
    use net_proto_api::encoder_api::Encoder;
    use net_proto_api::typed_api::Typed;

    use crate::api::network_graph::graph_edge::GraphEdgeDTO;
    use crate::api::network_graph::graph_node::GraphNodeDTO;
    use crate::api::network_graph::network_graph::NetworkGraphDTO;


    #[test]
    #[should_panic]
    fn reader_correctly_read_encoded_graph_edge() {
        const FIRST_NODE_ID: &str = "0.0.0.0:0000";
        const FIRST_NODE_AGENT_ID: &str = "some first node agent id";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ID, FIRST_NODE_AGENT_ID);
        const SECOND_NODE_ID: &str = "0.0.0.0:5656";
        const SECOND_NODE_AGENT_ID: &str = "some second node agent id";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ID, SECOND_NODE_AGENT_ID);

        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";
        let communication_types: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];

        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID, communication_types.as_slice());

        let network_graph = NetworkGraphDTO::new(
            &[first_graph_node, second_graph_node],
            &[graph_edge],
        );

        let mut binary_user_reader = ReaderBuilder::new().build(network_graph.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("graph_nodes", binary_user_reader.field_name().unwrap());
        
        binary_user_reader.step_in().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("node_id", binary_user_reader.field_name().unwrap());
        assert_eq!(FIRST_NODE_ID,  binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("agent_id", binary_user_reader.field_name().unwrap());
        assert_eq!(FIRST_NODE_AGENT_ID, binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("node_id", binary_user_reader.field_name().unwrap());
        assert_eq!(SECOND_NODE_ID,  binary_user_reader.read_string().unwrap().text());
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("agent_id", binary_user_reader.field_name().unwrap());
        assert_eq!(SECOND_NODE_AGENT_ID, binary_user_reader.read_string().unwrap().text());
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::List), binary_user_reader.next().unwrap());
        assert_eq!("graph_edges", binary_user_reader.field_name().unwrap());

        binary_user_reader.step_in().unwrap();

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
        binary_user_reader.step_out().unwrap();
        binary_user_reader.step_out().unwrap();
        // should panic here
        binary_user_reader.step_out().unwrap();
    }

    #[test]
    fn endec_network_graph() {
        const FIRST_NODE_ID: &str = "0.0.0.0:0000";
        const FIRST_NODE_AGENT_ID: &str = "first node agent id";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ID, FIRST_NODE_AGENT_ID);
        const SECOND_NODE_ID: &str = "0.0.0.0:5656";
        const SECOND_NODE_AGENT_ID: &str = "second node agent id";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ID, SECOND_NODE_AGENT_ID);

        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";

        let communication_types: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];

        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID, communication_types.as_slice());

        let network_graph = NetworkGraphDTO::new(
            &[first_graph_node, second_graph_node],
            &[graph_edge],
        );

        assert_eq!(network_graph, NetworkGraphDTO::decode(&network_graph.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const FIRST_NODE_ID: &str = "0.0.0.0:0000";
        const FIRST_NODE_AGENT_ID: &str = "first node agent id";
        let first_graph_node = GraphNodeDTO::new(FIRST_NODE_ID, FIRST_NODE_AGENT_ID);
        const SECOND_NODE_ID: &str = "0.0.0.0:5656";
        const SECOND_NODE_AGENT_ID: &str = "second node agent id";
        let second_graph_node = GraphNodeDTO::new(SECOND_NODE_ID, SECOND_NODE_AGENT_ID);

        const SRC_ID: &str = "0.0.0.0:0000";
        const DST_ID: &str = "0.0.0.0:5656";

        let communication_types: Vec<String> = vec!["fac1".to_string(), "fac2".to_string(), "fac3".to_string()];

        let graph_edge: GraphEdgeDTO = GraphEdgeDTO::new(SRC_ID, DST_ID, communication_types.as_slice());

        let network_graph = NetworkGraphDTO::new(
            &[first_graph_node, second_graph_node],
            &[graph_edge],
        );

        assert_eq!(network_graph.get_type(), NetworkGraphDTO::get_data_type());
        assert_eq!(network_graph.get_type(), super::DATA_TYPE);
    }
}