#[allow(dead_code)]

#[cfg(feature = "steven_protocol")]
extern crate steven_protocol;
#[cfg(feature = "steven_shared")]
extern crate steven_shared;
#[cfg(feature = "serde_json")]
extern crate serde_json;

#[macro_use]
pub mod protocol;
pub mod segment;