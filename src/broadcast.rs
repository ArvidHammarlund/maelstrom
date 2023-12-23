use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

use crate::{Address, MessageId, MessageIndex, MessageRegistry};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum BroadcastType {
    #[serde(rename = "broadcast")]
    Request,
    #[serde(rename = "broadcast_ok")]
    Response,
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait BroadcastHandler<A: Address, I: MessageIndex, T: Clone>:
    MessageId<I> + MessageRegistry<T>
{
    fn respond_broadcast(
        &mut self,
        incoming: &BroadcastRequest<I, T>,
    ) -> Result<BroadcastResponse<I>, crate::Error> {
        match incoming.kind {
            BroadcastType::Request => {
                let msg_id = self.gen_msg_id();
                self.push_msg(incoming.message.clone());
                Ok(BroadcastResponse {
                    kind: BroadcastType::Response,
                    in_reply_to: incoming.message_id.clone(),
                    message_id: msg_id,
                })
            }
            BroadcastType::Response => Err(crate::Error::MalformedRequest),
        }
    }
}

/// Generator message request message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BroadcastRequest<I: MessageIndex, T: Clone> {
    #[serde(rename = "type")]
    pub kind: BroadcastType,
    #[serde(rename = "msg_id")]
    pub message_id: I,
    pub message: T,
}

/// Generator message respond message
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BroadcastResponse<I> {
    #[serde(rename = "type")]
    pub kind: BroadcastType,
    pub in_reply_to: I,
    #[serde(rename = "msg_id")]
    pub message_id: I,
}

// #[cfg(test)]
// mod test {
//     use crate::{Message, MessageId, NodeId, ResponseBuilder};

//     use super::{BroadcastHandler, GeneratorRequest, GeneratorResponse};

//     #[derive(Default)]
//     pub struct TestNode {
//         n: u32,
//     }

//     impl MessageId<u32> for TestNode {
//         fn gen_msg_id(&mut self) -> u32 {
//             self.n += 1;
//             self.n
//         }
//     }

//     impl NodeId<String> for TestNode {
//         fn node_id(&self) -> String {
//             "n2".to_owned()
//         }

//         fn set_node_id(&self, id: String) -> Result<(), crate::Error> {
//             self.n = id;
//             Ok(())
//         }
//     }

//     impl BroadcastHandler<String, u32> for TestNode {}
//     impl ResponseBuilder<String, GeneratorRequest<u32>, GeneratorResponse<u32>> for TestNode {}

//     #[test]
//     fn test_parse_init() {
//         let request = r#"{
//           "src": "c1",
//           "dest": "n1",
//           "body": {
//             "type": "broadcast",
//             "msg_id": 1,
//             "generator": "Please generator 35"
//           }
//         } "#;
//         let mut test_node = TestNode::default();
//         let expected = r#"{"src":"n1","dest":"c1","body":{"type":"broadcast_ok","in_reply_to":1,"msg_id":1,"id":"n2-1"}}"#;
//         let request: Message<String, GeneratorRequest<u32>> =
//             serde_json::from_str(request).unwrap();
//         let response_body = test_node.respond_broadcast(&request.body).unwrap();
//         let response = TestNode::build_response(&request, response_body);
//         let res = serde_json::to_string(&response).unwrap();
//         assert_eq!(expected, res);
//     }
// }
