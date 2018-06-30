mod listen;

// reexported public interface
pub use self::listen::listen;

// utils
use i3ipc::{I3Connection, EstablishError};

pub fn connect() -> Result<I3Connection, EstablishError> {
    I3Connection::connect()
}
