use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

mod error;
pub use error::Error;

/// Unique identifier for a node
///
pub trait Address: Clone + Debug + Eq + PartialEq + Hash {}

impl Address for String {}
impl Address for &str {}

/// Internaly unique identifier for processed messages
///
pub trait MessageIndex: Clone + Debug + Eq + PartialEq + Hash + Ord + PartialOrd {}

impl MessageIndex for u32 {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum InitType {
    #[serde(rename = "init")]
    Request,
    #[serde(rename = "init_ok")]
    Response,
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait InitHandler<A: Address, I: MessageIndex> {
    fn respond_init(incoming: &InitRequest<A, I>) -> Result<InitResponse<I>, crate::Error> {
        match incoming.kind {
            InitType::Request => Ok(InitResponse {
                kind: InitType::Response,
                in_reply_to: incoming.message_id.clone(),
            }),
            InitType::Response => Err(crate::Error::MalformedRequest),
        }
    }
}

/// This trait determines the source address of outcoming packages
///
pub trait ResponseBuilder<A: Address, B1, B2> {
    fn build_response(request: &Message<A, B1>, new_body: B2) -> Message<A, B2> {
        Message {
            source: request.destination.clone(),
            destination: request.source.clone(),
            body: new_body,
        }
    }
}

/// Main communication medium for the network
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub struct Message<A: Address, B> {
    #[serde(rename = "src")]
    pub source: A,
    #[serde(rename = "dst")]
    pub destination: A,
    pub body: B,
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
    use serde_json::Value;

    use crate::{Address, InitHandler, InitRequest, Message, MessageIndex, ResponseBuilder};

    pub struct TestNode {}
    impl<A: Address, I: MessageIndex> InitHandler<A, I> for TestNode {}
    impl<A: Address, B1, B2> ResponseBuilder<A, B1, B2> for TestNode {}

    #[test]
    fn test_parse_init() {
        let request = r#" {
            "src": "321",
            "dst": "123",
            "body":{
              "type": "init",
              "msg_id":   1,
              "node_id":  "n3",
              "node_ids": ["n1", "n2", "n3"]
            }
        }"#;
        let expected = r#"{"src":"123","dst":"321","body":{"type":"init_ok","in_reply_to":1}}"#;
        dbg!(serde_json::from_str::<Value>(request)).unwrap();
        let request: Message<String, InitRequest<String, u32>> =
            serde_json::from_str(request).unwrap();
        dbg!(&request);
        let response_body = TestNode::respond_init(&request.body).unwrap();
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
