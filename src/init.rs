use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{Address, MessageIndex, NodeId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum InitType {
    #[serde(rename = "init")]
    Request,
    #[serde(rename = "init_ok")]
    Response,
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait InitHandler<A: Address, I: MessageIndex>: NodeId<A> {
    fn respond_init(
        &mut self,
        incoming: &InitRequest<A, I>,
    ) -> Result<InitResponse<I>, crate::Error> {
        match incoming.kind {
            InitType::Request => {
                self.set_node_id(incoming.node_id.clone())?;
                Ok(InitResponse {
                    kind: InitType::Response,
                    in_reply_to: incoming.message_id.clone(),
                })
            }
            InitType::Response => Err(crate::Error::MalformedRequest),
        }
    }
}

/// Init message request message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct InitRequest<A: Address, I: MessageIndex> {
    #[serde(rename = "type")]
    pub kind: InitType,
    #[serde(rename = "msg_id")]
    pub message_id: I,
    pub node_id: A,
    pub node_ids: Vec<A>,
}

/// Init message respond message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct InitResponse<I> {
    #[serde(rename = "type")]
    pub kind: InitType,
    pub in_reply_to: I,
}

#[cfg(test)]
mod test {

    use crate::{init::InitRequest, Message, MessageIndex, NodeId, ResponseBuilder};

    use super::InitHandler;

    pub struct TestNode {
        n: String,
    }

    impl NodeId<String> for TestNode {
        fn node_id(&self) -> String {
            "n2".to_owned()
        }

        fn set_node_id(&mut self, id: String) -> Result<(), crate::Error> {
            self.n = id;
            Ok(())
        }
    }

    impl<I: MessageIndex> InitHandler<String, I> for TestNode {}
    impl<B1, B2> ResponseBuilder<String, B1, B2> for TestNode {}

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
        let request: Message<String, InitRequest<String, u32>> =
            serde_json::from_str(request).unwrap();
        let response_body = node.respond_init(&request.body).unwrap();
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}