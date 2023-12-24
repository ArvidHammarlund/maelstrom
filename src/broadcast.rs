use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{error::Code, Address, MessageId, MessageIndex, MessageRegistry};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum BroadcastBody<I, T>
where
    I: MessageIndex,
{
    #[serde(rename = "broadcast")]
    PushRequest {
        #[serde(rename = "msg_id")]
        message_id: I,
        message: T,
    },
    #[serde(rename = "broadcast_ok")]
    PushResponse {
        in_reply_to: I,
        #[serde(rename = "msg_id")]
        message_id: I,
    },
    #[serde(rename = "read")]
    ReadRequest {
        #[serde(rename = "msg_id")]
        message_id: I,
    },
    #[serde(rename = "read_ok")]
    ReadResponse {
        #[serde(rename = "msg_id")]
        message_id: I,
        in_reply_to: I,
        messages: Vec<T>,
    },
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait BroadcastHandler<A, I, T>: MessageId<I> + MessageRegistry<T>
where
    A: Address,
    I: MessageIndex,
    T: Clone,
{
    fn respond_broadcast(
        &mut self,
        request: BroadcastBody<I, T>,
    ) -> Result<BroadcastBody<I, T>, crate::Error<I>> {
        match request {
            BroadcastBody::PushRequest {
                message,
                message_id,
            } => {
                let msg_id = self.gen_msg_id();
                self.push_msg(message);
                Ok(BroadcastBody::PushResponse {
                    in_reply_to: message_id,
                    message_id: msg_id,
                })
            }
            BroadcastBody::ReadRequest { message_id } => Ok(BroadcastBody::ReadResponse {
                in_reply_to: message_id,
                message_id: self.gen_msg_id(),
                messages: self.messages().to_owned(),
            }),
            BroadcastBody::PushResponse { message_id, .. } => Err(crate::Error::new(
                message_id,
                Code::MalformedRequest,
                "Request is response".to_owned(),
            )),
            BroadcastBody::ReadResponse { message_id, .. } => Err(crate::Error::new(
                message_id,
                Code::MalformedRequest,
                "Request is response".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        broadcast::BroadcastBody, Message, MessageId, MessageRegistry, NodeId, ResponseBuilder,
    };

    use super::BroadcastHandler;

    #[derive(Default)]
    pub struct TestNode {
        n: u32,
        id: String,
        messages: Vec<u32>,
    }

    impl MessageId<u32> for TestNode {
        fn gen_msg_id(&mut self) -> u32 {
            self.n += 1;
            self.n
        }
    }

    impl NodeId<String, u32> for TestNode {
        fn node_id(&self) -> &String {
            &self.id
        }

        fn set_node_id(&mut self, id: String) -> Result<(), crate::Error<u32>> {
            self.id = id;
            Ok(())
        }
    }

    impl MessageRegistry<u32> for TestNode {
        fn push_msg(&mut self, msg: u32) {
            self.messages.push(msg);
        }
        fn messages(&self) -> &[u32] {
            self.messages.as_slice()
        }
    }

    impl BroadcastHandler<String, u32, u32> for TestNode {}
    impl ResponseBuilder<String, u32, BroadcastBody<u32, u32>> for TestNode {}

    #[test]
    fn test_parse_broadcast() {
        let request = r#"{
          "src": "c1",
          "dest": "n1",
          "body": {
            "type": "broadcast",
            "message": 1000,
            "msg_id": 1
          }
        } "#;
        let mut test_node = TestNode::default();
        let expected =
            r#"{"src":"n1","dest":"c1","body":{"type":"broadcast_ok","in_reply_to":1,"msg_id":1}}"#;
        let request: Message<String, BroadcastBody<u32, u32>, u32> =
            serde_json::from_str(request).unwrap();
        let response_body = request
            .body
            .clone()
            .and_then(|body| test_node.respond_broadcast(body));
        let response = TestNode::build_response(&request, response_body);
        let res = serde_json::to_string(&response).unwrap();
        assert_eq!(expected, res);
    }

    #[test]
    fn test_broadcast_read() {
        let request = r#"{
          "src": "c1",
          "dest": "n1",
          "body": {
            "type": "broadcast",
            "message": 1000,
            "msg_id": 1
          }
        } "#;
        let read = r#"{
          "src": "c1",
          "dest": "n1",
          "body": {
            "type": "read",
            "msg_id": 1
          }
        } "#;
        let mut test_node = TestNode::default();
        let expected = r#"{"src":"n1","dest":"c1","body":{"type":"read_ok","msg_id":4,"in_reply_to":1,"messages":[1000,1000,1000]}}"#;
        let request: Message<String, BroadcastBody<u32, u32>, u32> =
            serde_json::from_str(request).unwrap();
        for _ in 0..3 {
            let _ = request
                .body
                .clone()
                .and_then(|body| test_node.respond_broadcast(body));
        }
        let read: Message<String, BroadcastBody<u32, u32>, u32> =
            serde_json::from_str(read).unwrap();
        let res = read
            .body
            .clone()
            .and_then(|body| test_node.respond_broadcast(body));
        let res = TestNode::build_response(&read, res);
        let res = serde_json::to_string(&res).unwrap();
        assert_eq!(expected, res);
    }
}
