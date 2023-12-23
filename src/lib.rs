mod error;
pub use error::Error;
pub mod broadcast;
pub mod echo;
pub mod generate;
pub mod init;

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Debug, hash::Hash};

/// Unique identifier for a node
///
pub trait Address: ToString + Clone + Debug + Eq + PartialEq + Hash {}

impl Address for String {}
impl Address for &str {}

/// Internaly unique identifier for processed messages
///
pub trait MessageIndex:
    ToString + Clone + Debug + Eq + PartialEq + Hash + Ord + PartialOrd
{
}

impl MessageIndex for u32 {}

pub trait NodeId<A: Address, I: MessageIndex> {
    fn set_node_id(&mut self, id: A) -> Result<(), crate::Error<I>>;
    fn node_id(&self) -> A;
}

pub trait MessageId<I: MessageIndex> {
    fn gen_msg_id(&mut self) -> I;
}

pub trait MessageRegistry<T> {
    fn push_msg(&mut self, msg: T);
    fn messages(&self) -> &[T];
}

pub trait TopologyRegistry<A: Address> {
    fn set_topology(&mut self, topology: Vec<A>);
}

/// Main communication medium for the network
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
pub struct Message<A: Address, B> {
    #[serde(rename = "src")]
    pub source: A,
    #[serde(rename = "dest")]
    pub destination: A,
    pub body: B,
}

/// This trait determines the source address of outcoming packages
///
pub trait ResponseBuilder<A: Address, I: MessageIndex, B> {
    fn build_response(
        request: &Message<A, B>,
        new_body: Result<B, crate::Error<I>>,
    ) -> Message<A, Result<B, crate::Error<I>>> {
        Message {
            source: request.destination.clone(),
            destination: request.source.clone(),
            body: new_body,
        }
    }
}

// // impl<A: Address, B, I: MessageIndex> Serialize for Message<A, Result<B, crate::Error<I>>> {
// //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// //     where
// //         S: Serializer,
// //     {
// //         let mut state = serializer.serialize_struct("Message", 3)?;
// //         state.serialize_field("src", &self.source)?;
// //         state.serialize_field("dest", &self.destination)?;
// //         match &self.body {
// //             Ok(value) => state.serialize_field("body", &value)?,
// //             Err(error) => state.serialize_field("body", error)?,
// //         }
// //         state.end()
// //     }
// // }

// // fn skip_result<'de, D, B, I>(d: Result<B, crate::Error<I>>) -> Result<B, D::Error>
// // where
// //     D: Deserializer<'de>,
// //     I: MessageIndex,
// // {

// // }

// fn skip_result<S, B, I: MessageIndex>(body: &B, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
// }
