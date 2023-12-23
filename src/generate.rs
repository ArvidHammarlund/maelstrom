use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{error::Code, Address, MessageId, MessageIndex, NodeId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum GenerateBody<I>
where
    I: MessageIndex,
{
    #[serde(rename = "generate")]
    Request {
        #[serde(rename = "msg_id")]
        message_id: I,
    },
    #[serde(rename = "generate_ok")]
    Response {
        in_reply_to: I,
        #[serde(rename = "msg_id")]
        message_id: I,
        id: String,
    },
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait GenerateHandler<A: Address, I: MessageIndex>: NodeId<A, I> + MessageId<I>
where
    A: Address,
    I: MessageIndex,
{
    fn respond_generate(
        &mut self,
        request: GenerateBody<I>,
    ) -> Result<GenerateBody<I>, crate::Error<I>> {
        match request {
            GenerateBody::Request { message_id } => {
                let new_message_id = self.gen_msg_id();
                let id = format!(
                    "{}-{}",
                    self.node_id().to_string(),
                    new_message_id.to_string()
                );
                Ok(GenerateBody::Response {
                    in_reply_to: message_id,
                    id,
                    message_id: new_message_id,
                })
            }
            GenerateBody::Response { message_id, .. } => Err(crate::Error::new(
                message_id,
                Code::MalformedRequest,
                "Request is a response".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Message, MessageId, NodeId, ResponseBuilder};

    use super::{GenerateBody, GenerateHandler};

    #[derive(Default)]
    pub struct TestNode {
        n: u32,
        id: String,
    }

    impl MessageId<u32> for TestNode {
        fn gen_msg_id(&mut self) -> u32 {
            self.n += 1;
            self.n
        }
    }

    impl NodeId<String, u32> for TestNode {
        fn node_id(&self) -> String {
            "n2".to_owned()
        }

        fn set_node_id(&mut self, _id: String) -> Result<(), crate::Error<u32>> {
            self.id = "123".to_owned();
            Ok(())
        }
    }

    impl GenerateHandler<String, u32> for TestNode {}
    impl ResponseBuilder<String, u32, GenerateBody<u32>> for TestNode {}

    #[test]
    fn test_parse_generate() {
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
        let request: Message<String, GenerateBody<u32>, u32> =
            serde_json::from_str(request).unwrap();
        let response_body = request
            .body
            .clone()
            .and_then(|body| test_node.respond_generate(body));
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
