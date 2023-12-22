use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

mod error;
pub use error::Error;

pub mod echo;
pub mod init;

mod msg;
pub use msg::{Address, Message, MessageIndex, ResponseBuilder};

pub trait IdGenerator<I: MessageIndex> {
    fn gen_id(&mut self) -> I;
}
