use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(i8)]
#[serde(tag = "op")]
pub enum OpCode {
    /// OpCode sent as the first message to a client
    Hello = 0,
    /// OpCode used to create GenServers
    Create {
        /// What index this GenServer should be created in
        index: String,
        /// The state of this new GenServer
        #[serde(with = "serde_bytes")]
        state: Vec<u8>
    } = 1,
    /// OpCode used to save state to a GenServer
    Save {
        /// The PID to update
        pid: String,
        /// The new state of this PID
        #[serde(with = "serde_bytes")]
        state: Vec<u8>
    } = 2,
    /// OpCode used to delete a living GenServer
    Delete {
        /// The GenServer PID to delete
        pid: String
    } = 3,
    /// OpCode indicating success - Sent rarely
    Success = 4,
    /// OpCode indicating failure - also sent rarely. Timeouts should be preferred.
    Failure = 5,
    /// Sent by the Client to update which indexes
    /// Can be request to this Client.
    IndexUpdate {
        /// The indexes to allow requests from
        indexes: Vec<String>
    } = 6,
    /// OpCode used to send a message to a Process.
    Send {
        /// The PID to send to
        pid: String,
        /// The message to send
        #[serde(with = "serde_bytes")]
        message: Vec<u8>
    } = 7,
    /// OpCode sent when a process sends a message.
    Receive {
        /// The originating PID
        origin: String,
        /// The message sent from `origin`
        #[serde(with = "serde_bytes")]
        message: Vec<u8>
    } = 8,
    /// OpCode used for other Servers to subscribe to a GenServer's state updates.
    Subscribe {
        /// The GenServer to subscribe State updates with
        pid: String
    } = 9,
    /// OpCode used when a new state is published
    Publish {
        /// The originating GenServer PID of this State publish
        pid: String,
        /// The new state
        #[serde(with = "serde_bytes")]
        state: Vec<u8>
    } = 10
}
