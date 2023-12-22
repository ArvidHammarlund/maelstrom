mod error;
pub use error::Error;

pub mod broadcast;
pub mod echo;
pub mod generate;
pub mod init;

mod msg;
pub use msg::{Address, Message, MessageIndex, ResponseBuilder};

pub trait NodeId<A: Address> {
    fn set_node_id(&mut self, id: A) -> Result<(), crate::Error>;
    fn node_id(&self) -> A;
}

pub trait MessageId<I: MessageIndex> {
    fn gen_msg_id(&mut self) -> I;
}

pub trait MessageRegistry<T> {
    fn push_msg(&mut self, msg: T);
    fn messages(&self) -> &[&T];
}

pub trait TopologyRegistry<A: Address> {
    fn set_topology(&mut self, topology: Vec<A>);
}
