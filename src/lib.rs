use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Unique identifier for a node
///
pub trait Address: Clone + Debug + Eq + PartialEq + Hash {}

impl Address for String {}
impl Address for &str {}

/// Internaly unique identifier for processed messages
///
pub trait MessageIndex: Clone + Debug + Eq + PartialEq + Hash + Ord + PartialOrd {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum InitType {
    #[serde(rename = "init_ok")]
    Request,
    #[serde(rename = "init")]
    Response,
}

/// This trait has to be implement for every Node alongside any workload specific functionality
///
pub trait InitHandler<A: Address, I: MessageIndex> {
    fn respond_init(&self, incoming: InitRequest<A, I>) -> Result<InitRespond<I>, ()> {
        Ok(InitRespond {
            kind: incoming.kind,
            in_reply_to: incoming.message_id,
        })
    }
}

/// Main communication medium for the network
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
struct Message<A: Address, B> {
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
pub struct InitRespond<I> {
    #[serde(rename = "type")]
    pub kind: InitType,
    pub in_reply_to: I,
}

#[cfg(test)]
mod test {}
