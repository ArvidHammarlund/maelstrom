use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{error::Code, Address, MessageId, MessageIndex, NodeId, TopologyRegistry};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TopologyBody<I: MessageIndex, A: Address>
where
    A: Address,
    I: MessageIndex,
{
    #[serde(rename = "topology")]
    Request {
        #[serde(rename = "msg_id")]
        message_id: I,
        topology: HashMap<A, Vec<A>>,
    },
    #[serde(rename = "topology_ok")]
    Response {
        in_reply_to: I,
        #[serde(rename = "msg_id")]
        message_id: I,
    },
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait TopologyHandler<A, I>: MessageId<I> + NodeId<A, I> + TopologyRegistry<A>
where
    A: Address,
    I: MessageIndex,
{
    fn respond(
        &mut self,
        request: TopologyBody<I, A>,
    ) -> Result<TopologyBody<I, A>, crate::Error<I>> {
        match request {
            TopologyBody::Request {
                message_id,
                topology,
            } => {
                self.set_topology(topology.get(self.node_id()).unwrap().clone());
                Ok(TopologyBody::Response {
                    in_reply_to: message_id,
                    message_id: self.gen_msg_id(),
                })
            }
            TopologyBody::Response { message_id, .. } => Err(crate::Error::new(
                message_id,
                Code::MalformedRequest,
                "Request is response".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Address, Message, MessageId, NodeId, ResponseBuilder, TopologyRegistry};

    use super::{TopologyBody, TopologyHandler};

    #[derive(Default)]
    pub struct TestNode<A: Address> {
        n: u32,
        topology: Vec<A>,
        id: A,
    }

    impl MessageId<u32> for TestNode<String> {
        fn gen_msg_id(&mut self) -> u32 {
            self.n += 1;
            self.n
        }
    }

    impl NodeId<String, u32> for TestNode<String> {
        fn node_id(&self) -> &String {
            &self.id
        }

        fn set_node_id(&mut self, _id: String) -> Result<(), crate::Error<u32>> {
            self.id = "123".to_owned();
            Ok(())
        }
    }

    impl TopologyHandler<String, u32> for TestNode<String> {}
    impl ResponseBuilder<String, u32, TopologyBody<u32, String>> for TestNode<String> {}
    impl TopologyRegistry<String> for TestNode<String> {
        fn set_topology(&mut self, topology: Vec<String>) {
            self.topology = topology;
        }
    }

    #[test]
    fn test_parse_topology() {
        let request = r#"{
          "src": "c1",
          "dest": "n1",
          "body": {
            "type": "topology",
            "msg_id": 1,
            "topology": {
                "n1": ["n2", "n3"],
                "n2": ["n1"],
                "n3": ["n1"]
              }
          }
        } "#;
        let mut test_node = TestNode {
            n: 0,
            topology: Vec::new(),
            id: "n2".to_owned(),
        };
        let expected =
            r#"{"src":"n1","dest":"c1","body":{"type":"topology_ok","in_reply_to":1,"msg_id":1}}"#;
        let request: Message<String, TopologyBody<u32, String>, u32> =
            serde_json::from_str(request).unwrap();
        let response_body = request
            .body
            .clone()
            .and_then(|body| test_node.respond(body));
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
