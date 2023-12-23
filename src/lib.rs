mod error;
pub use error::Error;
pub mod broadcast;
pub mod echo;
pub mod generate;
pub mod init;

use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Message<A: Address, B, I: MessageIndex> {
    #[serde(rename = "src")]
    pub source: A,
    #[serde(rename = "dest")]
    pub destination: A,
    pub body: Result<B, crate::Error<I>>,
}

/// This trait determines the source address of outcoming packages
///
pub trait ResponseBuilder<A: Address, I: MessageIndex, B> {
    fn build_response(
        request: &Message<A, B, I>,
        new_body: Result<B, crate::Error<I>>,
    ) -> Message<A, B, I> {
        Message {
            source: request.destination.clone(),
            destination: request.source.clone(),
            body: new_body,
        }
    }
}
