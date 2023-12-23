use crate::MessageIndex;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
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

#[derive(thiserror::Error, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone, new)]
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
