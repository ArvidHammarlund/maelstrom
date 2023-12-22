use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{Address, MessageId, MessageIndex, NodeId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum GenerateType {
    #[serde(rename = "generate")]
    Request,
    #[serde(rename = "generate_ok")]
    Response,
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait GenerateHandler<A: Address, I: MessageIndex>: NodeId<A> + MessageId<I> {
    fn respond_generate(
        &mut self,
        incoming: &GeneratorRequest<I>,
    ) -> Result<GeneratorResponse<I>, crate::Error> {
        match incoming.kind {
            GenerateType::Request => {
                let msg_id = self.gen_msg_id();
                Ok(GeneratorResponse {
                    kind: GenerateType::Response,
                    in_reply_to: incoming.message_id.clone(),
                    id: format!("{}-{}", self.node_id().to_string(), msg_id.to_string()),
                    message_id: msg_id,
                })
            }
            GenerateType::Response => Err(crate::Error::MalformedRequest),
        }
    }
}

/// Generator message request message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GeneratorRequest<I: MessageIndex> {
    #[serde(rename = "type")]
    pub kind: GenerateType,
    #[serde(rename = "msg_id")]
    pub message_id: I,
}

/// Generator message respond message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GeneratorResponse<I> {
    #[serde(rename = "type")]
    pub kind: GenerateType,
    pub in_reply_to: I,
    #[serde(rename = "msg_id")]
    pub message_id: I,
    pub id: String,
}

#[cfg(test)]
mod test {
    use crate::{Message, MessageId, NodeId, ResponseBuilder};

    use super::{GenerateHandler, GeneratorRequest, GeneratorResponse};

    #[derive(Default)]
    pub struct TestNode {
        n: u32,
    }

    impl MessageId<u32> for TestNode {
        fn gen_msg_id(&mut self) -> u32 {
            self.n += 1;
            self.n
        }
    }

    impl NodeId<String> for TestNode {
        fn node_id(&self) -> String {
            "n2".to_owned()
        }

        fn set_node_id(&self, id: String) -> Result<(), crate::Error> {
            self.n = id;
            Ok(())
        }
    }

    impl GenerateHandler<String, u32> for TestNode {}
    impl ResponseBuilder<String, GeneratorRequest<u32>, GeneratorResponse<u32>> for TestNode {}

    #[test]
    fn test_parse_init() {
        let request = r#"{
          "src": "c1",
          "dest": "n1",
          "body": {
            "type": "generate",
            "msg_id": 1,
            "generator": "Please generator 35"
          }
        } "#;
        let mut test_node = TestNode::default();
        let expected = r#"{"src":"n1","dest":"c1","body":{"type":"generate_ok","in_reply_to":1,"msg_id":1,"id":"n2-1"}}"#;
        let request: Message<String, GeneratorRequest<u32>> =
            serde_json::from_str(request).unwrap();
        let response_body = test_node.respond_generate(&request.body).unwrap();
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
