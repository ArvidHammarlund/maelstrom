use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Unique identifier for a node
///
pub trait Address: Clone + Debug + Eq + PartialEq + Hash {}

impl Address for String {}
impl Address for &str {}

/// Internaly unique identifier for processed messages
///
pub trait MessageIndex: Clone + Debug + Eq + PartialEq + Hash + Ord + PartialOrd {}

impl MessageIndex for u32 {}

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
    #[serde(rename = "dest")]
    pub destination: A,
    pub body: B,
}
