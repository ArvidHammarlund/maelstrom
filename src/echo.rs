use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{Address, MessageId, MessageIndex};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EchoBody<I> {
    #[serde(rename = "echo")]
    Request {
        #[serde(rename = "msg_id")]
        message_id: I,
        echo: String,
    },
    #[serde(rename = "echo_ok")]
    Response {
        in_reply_to: I,
        #[serde(rename = "msg_id")]
        message_id: I,
        echo: String,
    },
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait EchoHandler<A: Address, I: MessageIndex>: MessageId<I> {
    fn respond_echo(&mut self, request: EchoBody<I>) -> Result<EchoBody<I>, crate::Error> {
        match request {
            EchoBody::Request { message_id, echo } => Ok(EchoBody::Response {
                in_reply_to: message_id,
                message_id: self.gen_msg_id(),
                echo,
            }),
            _ => Err(crate::Error::MalformedRequest),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Message, MessageId, ResponseBuilder};

    use super::{EchoBody, EchoHandler};

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
    impl ResponseBuilder<String, EchoBody<u32>> for TestNode {}

    #[test]
    fn test_parse_echo() {
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
        let request: Message<String, EchoBody<u32>> = serde_json::from_str(request).unwrap();
        let response_body = test_node.respond_echo(request.body.clone());
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }
}
