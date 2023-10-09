use ion_rs;
use ion_rs::IonWriter;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;

const DATA_TYPE: &str = "graph_node";
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GraphNodeDTO {
    node_id: String,
    agent_id: String,
}

impl net_proto_api::api::API for GraphNodeDTO { }

impl GraphNodeDTO {
    pub fn new(node_id: &str, agent_id: &str) -> Self {
        GraphNodeDTO {
            node_id: node_id.into(),
            agent_id: agent_id.into(),
        }
    }

    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }

    pub fn get_agent_id(&self) -> &str {
        &self.agent_id
    }
}

impl Encoder for GraphNodeDTO {
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
        
        writer.set_field_name("node_id");
        writer.write_string(&self.node_id).unwrap();

        writer.set_field_name("agent_id");
        writer.write_string(&self.agent_id).unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
}

impl Decoder for GraphNodeDTO {
    fn decode(data: &[u8]) -> Self {

        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let node_id = binding.text();

        binary_user_reader.next().unwrap();
        let binding = binary_user_reader.read_string().unwrap();
        let agent_id = binding.text();

        GraphNodeDTO::new(
            node_id,
            agent_id
        )
    }
}

impl net_proto_api::typed_api::Typed for GraphNodeDTO {
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

    use crate::api::network_graph::graph_node::GraphNodeDTO;


    #[test]
    fn reader_correctly_read_encoded_graph_node() {
        const NODE_ID: &str = "0.0.0.0:0000";
        const AGENT_ID: &str = "some-agent-id";

        let graph_node = GraphNodeDTO::new(NODE_ID, AGENT_ID);
        let mut binary_user_reader = ReaderBuilder::new().build(graph_node.encode()).unwrap();

        assert_eq!(StreamItem::Value(IonType::Struct), binary_user_reader.next().unwrap());
        binary_user_reader.step_in().unwrap();
        
        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("node_id", binary_user_reader.field_name().unwrap());
        assert_eq!(NODE_ID, binary_user_reader.read_string().unwrap().text());

        assert_eq!(StreamItem::Value(IonType::String), binary_user_reader.next().unwrap());
        assert_eq!("agent_id", binary_user_reader.field_name().unwrap());
        assert_eq!(AGENT_ID, binary_user_reader.read_string().unwrap().text());
    }

    #[test]
    fn endec_graph_node() {
        const NODE_ID: &str = "0.0.0.0:0000";
        const AGENT_ID: &str = "some-agent-id";

        let graph_node = GraphNodeDTO::new(NODE_ID, AGENT_ID);
        let endec_graph_node = GraphNodeDTO::decode(&graph_node.encode());
        assert_eq!(graph_node, GraphNodeDTO::decode(&graph_node.encode()));
    }

    #[test]
    fn test_getting_data_types() {
        const NODE_ID: &str = "0.0.0.0:0000";
        const AGENT_ID: &str = "some-agent-id";

        let graph_node = GraphNodeDTO::new(NODE_ID, AGENT_ID);
        assert_eq!(graph_node.get_type(), GraphNodeDTO::get_data_type());
        assert_eq!(graph_node.get_type(), super::DATA_TYPE);
    }
}