#[allow(dead_code)]

#[cfg(feature = "steven_protocol")]
extern crate steven_protocol;
#[cfg(feature = "steven_protocol")]
extern crate steven_shared;
#[cfg(feature = "serde_json")]
extern crate serde_json;

#[macro_use]
pub mod protocol;
pub mod segment;

#[cfg(test)]
mod tests {
    use crate::protocol::{State};
    use crate::protocol::Direction;

    #[test]
    fn test_define_protocol() {
        define_protocol!(pub Proto_1_17_1, "1.17.1", 755, {
            State::Handshaking => {
                Direction::ClientBound => {
                    0x00 => HandshakingPacket{
                        test: u8,
                        test2: u8 where |s|{s.test2 > 1},
                        test3: u8,
                    },
                    0x01 => TestPacket
                },
                Direction::ServerBound => {
                    0x00 => HandshakingPacketTwo{
                        test: u8,
                        test2: u8 where |s|{s.test2 > 1},
                        test3: u8,
                    },
                    0x01 => TestPacketTwo
                },
            },
            State::Status => {
                Direction::ClientBound => {
                    0x00 => StatusPlaceholder
                }
            },
            State::Login => {
                Direction::ClientBound => {
                    0x00 => LoginPlaceholder
                }
            },
            State::Play => {
                Direction::ClientBound => {
                    0x00 => PlayPlaceholder
                }
            },
        });
    }
}