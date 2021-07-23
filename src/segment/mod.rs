pub mod implementation;

pub trait Segment: Default{
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>;
    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
}