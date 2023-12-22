mod error;
pub use error::Error;

pub mod echo;
pub mod generate;
pub mod init;

mod msg;
pub use msg::{Address, Message, MessageIndex, ResponseBuilder};

pub trait Node<A: Address, I: MessageIndex> {
    fn node_id(&self) -> A;
    fn gen_msg_id(&mut self) -> I;
}
