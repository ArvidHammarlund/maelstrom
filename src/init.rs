use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{error::Code, Address, MessageIndex, NodeId};

/// Body for initialization messages
///
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum InitBody<I, A>
where
    I: MessageIndex,
    A: Address,
{
    /// Init message request message
    #[serde(rename = "init")]
    Request {
        #[serde(rename = "msg_id")]
        message_id: I,
        node_id: A,
        node_ids: Vec<A>,
    },
    /// Init response request message
    #[serde(rename = "init_ok")]
    Response { in_reply_to: I },
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait InitHandler<A, I>: NodeId<A, I>
where
    A: Address,
    I: MessageIndex,
{
    fn respond_init(&mut self, request: InitBody<I, A>) -> Result<InitBody<I, A>, crate::Error<I>> {
        match request {
            InitBody::Request {
                node_id,
                message_id,
                ..
            } => {
                self.set_node_id(node_id)?;
                Ok(InitBody::Response {
                    in_reply_to: message_id,
                })
            }
            InitBody::Response { in_reply_to } => Err(crate::Error::new(
                in_reply_to,
                Code::MalformedRequest,
                "Request is a response".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{init::InitBody, Message, NodeId, ResponseBuilder};

    use super::InitHandler;

    pub struct TestNode {
        n: String,
    }

    impl NodeId<String, u32> for TestNode {
        fn node_id(&self) -> String {
            "n2".to_owned()
        }

        fn set_node_id(&mut self, id: String) -> Result<(), crate::Error<u32>> {
            self.n = id;
            Ok(())
        }
    }

    impl InitHandler<String, u32> for TestNode {}
    impl ResponseBuilder<String, u32, InitBody<u32, String>> for TestNode {}

    #[test]
    fn test_parse_init() {
        let request = r#" {
            "src": "321",
            "dest": "123",
            "body":{
              "type": "init",
              "msg_id":   1,
              "node_id":  "n3",
              "node_ids": ["n1", "n2", "n3"]
            }
        }"#;
        let mut node = TestNode {
            n: "hello".to_owned(),
        };
        let expected = r#"{"src":"123","dest":"321","body":{"type":"init_ok","in_reply_to":1}}"#;
        let request: Message<String, InitBody<u32, String>, u32> =
            serde_json::from_str(request).unwrap();
        let response_body = request
            .body
            .clone()
            .and_then(|body| node.respond_init(body));
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
