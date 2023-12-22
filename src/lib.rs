use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Unique identifier for a node
///
pub trait Address {}

impl Address for String {}
impl Address for &str {}

pub trait Body {}

pub struct Node {}

/// Main communication medium for network
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
struct Message<A: Address, B: Body> {
    #[serde(rename = "src")]
    pub source: A,
    #[serde(rename = "src")]
    pub destination: A,
    #[serde(rename = "src")]
    pub body: B,
}

/// Init message body
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Default)]
struct InitRequest<A: Address> {
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "msg_id")]
    message_id: A,
    #[serde()]
}
