use crate::segment::Segment;

#[derive(Debug, Clone)]
pub enum State{
    Handshaking,
    Status,
    Login,
    Play
}

#[derive(Debug, Clone)]
pub enum Direction{
    ClientBound,
    ServerBound
}

pub trait Protocol: Sized{
    fn name() -> &'static str;
    fn protocol() -> i32;
    fn packet_by_id<R: std::io::Read>(state: State, direction: Direction, id: i32, reader: &mut R) -> Option<std::io::Result<Self>>;
}

pub trait Packet: Segment + Sized{
    const PACKET_ID: i32;
    #[inline]
    fn packet_id(&self) -> i32 {
        Self::PACKET_ID
    }
}

#[allow(unused)]
macro_rules! define_protocol {
    ($struct_vis:vis $struct_name:ident, $protocol_name:literal, $protocol_version:literal, {$($state:path =>{$($direction:path =>{$($(#[$struct_doc:meta])* $id:literal => $packet:ident$({$( $(#[$field_doc:meta])* $field:ident: $value_type:ty $(where |$acceptor:ident|$condition:block)?),*$(,)?})?),+$(,)?}),+$(,)?}),+$(,)?}) => {

        $($($(
        #[allow(unused)]
        #[derive(Debug, Default)]
        $(#[$struct_doc])*
        $struct_vis struct $packet {
            $($(
                $(#[$field_doc])*
                pub $field: $value_type
            ),*)*
        }

        impl crate::protocol::Packet for $packet{
            const PACKET_ID: i32 = $id;
        }

        impl crate::segment::Segment for $packet {
            #[allow(unused)]
            fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
                $($(self.$field = {
                    let mut field: $value_type = Default::default();
                    $(if (|$acceptor: &Self|$condition)(self))?
                       {crate::segment::Segment::read_from_stream(&mut field, reader)?;}
                    field
                };)*)*
                Ok(())
            }
            #[allow(unused)]
            fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>{
                $($($(if (|$acceptor: &Self|$condition)(self))?
                   { crate::segment::Segment::write_to_stream(&self.$field, writer)?; }
                )*)*
                Ok(())
            }
        })+)+)+

        #[allow(unused, non_camel_case_types)]
        $struct_vis enum $struct_name {
            $($($($packet(Box<$packet>)),+),+),+
        }

        impl crate::protocol::Protocol for $struct_name {
            #[allow(unused)]
            fn name() -> &'static str {
                $protocol_name
            }

            #[allow(unused)]
            fn protocol() -> i32 {
                $protocol_version
            }

            #[allow(unused)]
            fn packet_by_id<R: std::io::Read>(state: State, direction: crate::protocol::Direction, id: i32, reader: &mut R) -> Option<std::io::Result<Self>> {
                match state {
                    $($state => {
                        match direction {
                            $($direction => {
                                match id {
                                    $($id => {
                                        let mut p: Box<$packet> = Box::new(Default::default());
                                        if let Err(e) = crate::segment::Segment::read_from_stream(&mut p, reader){
                                            Some(Err(e))
                                        }else{
                                            Some(Ok(Self::$packet(p)))
                                        }
                                    }),+,
                                    _ => None
                                }
                            }),+,
                            _ => None
                        }
                    }),+,
                    _ => None
                }
            }
        }
    };
}
