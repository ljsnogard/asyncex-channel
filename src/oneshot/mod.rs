//! This mod contains the `Oneshot` struct and its producer and consumer types,
//! `Sender`, `Receiver` and `Peeker`.

mod oneshot_;
mod peeker_;
mod recver_;
mod sender_;

pub use oneshot_::Oneshot;
pub use peeker_::{Peeker, PeekAsync};
pub use recver_::{Receiver, RxError};
pub use sender_::{Sender, TxError};
