use crate::segment::Segment;
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};

impl Segment for bool {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        *self = reader.read_u8()? != 0;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

/*
    Unsigned integers
 */
impl Segment for u8 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_u8()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u8(*self)?;
        Ok(())
    }
}

impl Segment for u16 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_u16::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Segment for u32 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_u32::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Segment for u64 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_u64::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u64::<BigEndian>(*self)?;
        Ok(())
    }
}

/*
    Signed integers
 */
impl Segment for i8 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_i8()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_i8(*self)?;
        Ok(())
    }
}

impl Segment for i16 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_i16::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_i16::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Segment for i32 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_i32::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_i32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Segment for i64 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_i64::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}

/*
    FLOATS
 */
impl Segment for f32 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_f32::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_f32::<BigEndian>(*self)?;
        Ok(())
    }
}

impl Segment for f64 {
    fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()>{
        *self = reader.read_f64::<BigEndian>()?;
        Ok(())
    }

    fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_f64::<BigEndian>(*self)?;
        Ok(())
    }
}