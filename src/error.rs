#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Indicates that the requested operation could not be completed within a timeout.
    #[error("requested operation could not be completed")]
    Timeout = 0,
    /// Thrown when a client sends an RPC request to a node which does not exist.
    #[error("Node does not exist")]
    NodeNotFound = 1,
    /// Use this error to indicate that a requested operation is not supported by the current implementation.
    #[error("requested operation is not supported")]
    NotSupported = 10,
    /// Indicates that the operation definitely cannot be performed at this time --
    /// perhaps because the server is in a read-only state, has not yet been initialized, believes its peers to be down, and so on.
    /// Do not use this error for indeterminate cases, when the operation may actually have taken place.
    #[error("The current node of the state cannot serve requested operation")]
    TemporarilyUnavailable = 11,
    /// The client's request did not conform to the server's expectations, and could not possibly have been processed.
    #[error("request did not conform to the server's expectations, and could not possibly have been processed")]
    MalformedRequest = 12,
    /// Indicates that some kind of general, indefinite error occurred.
    /// Use this as a catch-all for errors you can't otherwise categorize, or as a starting point for your error handler:
    /// it's safe to return internal-error for every problem by default, then add special cases for more specific errors later.
    #[error("node crashed")]
    Crash = 13,
    /// Indicates that some kind of general, definite error occurred. Use this as a catch-all for errors you can't otherwise categorize,
    /// when you specifically know that the requested operation has not taken place.
    /// For instance, you might encounter an indefinite failure during the prepare phase of a transaction: since you haven't started the commit process yet,
    /// the transaction can't have taken place. It's therefore safe to return a definite abort to the client.
    #[error("Indicates that some kind of general, definite error occurred")]
    Abort = 14,
    /// The client requested an operation on a key which does not exist (assuming the operation should not automatically create missing keys).
    #[error("requested an operation on a key which does not exist")]
    KeyDoesNotExist = 20,
    /// The client requested the creation of a key which already exists, and the server will not overwrite it.
    #[error("requested the creation of a key which already exists")]
    KeyAlreadyExists = 21,
    /// The requested operation expected some conditions to hold, and those conditions were not met.
    /// For instance, a compare-and-set operation might assert that the value of a key is currently 5; if the value is 3, the server would return precondition-failed.
    #[error("The requested operation expected some conditions to hold, and those conditions were not met.")]
    PreconditionFailed = 22,
    /// The requested transaction has been aborted because of a conflict with another transaction.
    /// Servers need not return this error on every conflict: they may choose to retry automatically instead.
    #[error(
        "The requested transaction has been aborted because of a conflict with another transaction"
    )]
    TxnConflict = 30,
}
