use crate::MessageIndex;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[serde(try_from = "u32")]
pub enum Code {
    Timeout = 0,
    NodeNotFound = 1,
    NotSupported = 10,
    TemporarilyUnavailable = 11,
    MalformedRequest = 12,
    Crash = 13,
    Abort = 14,
    KeyDoesNotExist = 20,
    KeyAlreadyExists = 21,
    PreconditionFailed = 22,
    TxnConflict = 30,
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.clone() as u32)
    }
}

impl TryFrom<u32> for Code {
    type Error = &'static str;
    fn try_from(x: u32) -> Result<Code, Self::Error> {
        num_traits::FromPrimitive::from_u32(x).ok_or("invalid value for code field")
    }
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
#[serde(tag = "type", rename = "error")]
pub struct Error<I: MessageIndex> {
    in_reply_to: I,
    code: Code,
    #[serde(rename = "text")]
    msg: String,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_deserialize_error() {
        let json = r#"{
         "type":        "error",
         "in_reply_to": 5,
         "code":        11,
         "text":        "Node n5 is waiting for quorum and cannot service requests yet"
        }"#;
        let expected = Error::<u32> {
            in_reply_to: 5,
            code: Code::TemporarilyUnavailable,
            msg: "Node n5 is waiting for quorum and cannot service requests yet".to_owned(),
        };
        let parsed: Result<Error<u32>, serde_json::Error> = serde_json::from_str(json);
        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn test_serialize_error() {
        let error = Error::<u32> {
            in_reply_to: 5,
            code: Code::TemporarilyUnavailable,
            msg: "Node n5 is waiting for quorum and cannot service requests yet".to_owned(),
        };
        let expected = r#"{"type":"error","in_reply_to":5,"code":11,"text":"Node n5 is waiting for quorum and cannot service requests yet"}"#;
        let serialized = serde_json::to_string(&error);
        assert_eq!(serialized.unwrap(), expected);
    }
}
