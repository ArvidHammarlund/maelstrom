use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{Address, MessageId, MessageIndex};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum EchoType {
    #[serde(rename = "echo")]
    Request,
    #[serde(rename = "echo_ok")]
    Response,
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait EchoHandler<A: Address, I: MessageIndex>: MessageId<I> {
    fn respond_echo(&mut self, incoming: &EchoRequest<I>) -> Result<EchoResponse<I>, crate::Error> {
        match incoming.kind {
            EchoType::Request => Ok(EchoResponse {
                kind: EchoType::Response,
                in_reply_to: incoming.message_id.clone(),
                message_id: self.gen_msg_id(),
                echo: incoming.echo.clone(),
            }),
            EchoType::Response => Err(crate::Error::MalformedRequest),
        }
    }
}

/// Echo message request message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct EchoRequest<I: MessageIndex> {
    #[serde(rename = "type")]
    pub kind: EchoType,
    #[serde(rename = "msg_id")]
    pub message_id: I,
    pub echo: String,
}

/// Echo message respond message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct EchoResponse<I> {
    #[serde(rename = "type")]
    pub kind: EchoType,
    pub in_reply_to: I,
    #[serde(rename = "msg_id")]
    pub message_id: I,
    pub echo: String,
}

#[cfg(test)]
mod test {
    use crate::{Message, MessageId, ResponseBuilder};

    use super::{EchoHandler, EchoRequest, EchoResponse};

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

    impl EchoHandler<String, u32> for TestNode {}
    impl ResponseBuilder<String, EchoRequest<u32>, EchoResponse<u32>> for TestNode {}

    #[test]
    fn test_parse_init() {
        let request = r#"{
          "src": "c1",
          "dest": "n1",
          "body": {
            "type": "echo",
            "msg_id": 1,
            "echo": "Please echo 35"
          }
        } "#;
        let mut test_node = TestNode::default();
        let expected = r#"{"src":"n1","dest":"c1","body":{"type":"echo_ok","in_reply_to":1,"msg_id":1,"echo":"Please echo 35"}}"#;
        let request: Message<String, EchoRequest<u32>> = serde_json::from_str(request).unwrap();
        let response_body = test_node.respond_echo(&request.body).unwrap();
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
