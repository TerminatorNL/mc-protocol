use crate::segment::Segment;
use std::ops::{DerefMut, Deref};

pub mod num;
pub mod mojang;
#[cfg(feature = "steven_protocol")]
pub mod steven;

impl<T: Segment> Segment for Box<T>{
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.deref_mut().read_from_stream(reader)
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.deref().write_to_stream(writer)
    }
}

impl<T: Segment> Segment for Option<T>{
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        let mut t = Default::default();
        T::read_from_stream(&mut t, reader)?;
        *self = Some(t);
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if let Some(inner) = self{
            inner.write_to_stream(writer)
        }else{
            Ok(())
        }
    }
}