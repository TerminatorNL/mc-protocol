/// Implements Stevenarella to be used in this project.
/// https://github.com/iceiix/stevenarella
pub mod version;

mod private {
    fn convert_error(steven_error: steven_protocol::protocol::Error) -> std::io::Error{
        std::io::Error::new(std::io::ErrorKind::Other, steven_error)
    }

    /// This macro is a workaround because sealed traits do not exist yet.
    macro_rules! impl_serialize {
        ($struct_name:path) => {
            impl crate::segment::Segment for $struct_name{
                fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
                    *self = steven_protocol::protocol::Serializable::read_from(reader).map_err(convert_error)?;
                    Ok(())
                }

                fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    steven_protocol::protocol::Serializable::write_to(self, writer).map_err(convert_error)?;
                    Ok(())
                }
            }
        };
        (optional $struct_name:path) => {
            impl crate::segment::Segment for Option<$struct_name>{
                fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
                    *self = steven_protocol::protocol::Serializable::read_from(reader).map_err(convert_error)?;
                    Ok(())
                }

                fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    steven_protocol::protocol::Serializable::write_to(self, writer).map_err(convert_error)?;
                    Ok(())
                }
            }
        };
        ($target_name:ident, $generic_trait:path) => {
            impl<T: $generic_trait + Default> crate::segment::Segment for $target_name<T>{
                fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
                    *self = steven_protocol::protocol::Serializable::read_from(reader).map_err(convert_error)?;
                    Ok(())
                }

                fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    steven_protocol::protocol::Serializable::write_to(self, writer).map_err(convert_error)?;
                    Ok(())
                }
            }
        };
        ($target_name:ident, $generic_trait:path, $generic_trait2:path) => {
            impl<T: $generic_trait, TT: $generic_trait2 + Default> crate::segment::Segment for $target_name<T,TT>{
                fn read_from_stream<R: std::io::Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
                    *self = steven_protocol::protocol::Serializable::read_from(reader).map_err(convert_error)?;
                    Ok(())
                }

                fn write_to_stream<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    steven_protocol::protocol::Serializable::write_to(self, writer).map_err(convert_error)?;
                    Ok(())
                }
            }
        }
    }

    impl_serialize!(steven_protocol::protocol::VarInt);
    impl_serialize!(steven_protocol::protocol::VarShort);
    impl_serialize!(steven_protocol::protocol::VarLong);
    impl_serialize!(steven_protocol::format::Component);
    impl_serialize!(steven_protocol::protocol::UUID);
    impl_serialize!(optional steven_protocol::nbt::NamedTag);
    impl_serialize!(optional steven_protocol::item::Stack);
    impl_serialize!(optional steven_protocol::nbt::Tag);
    impl_serialize!(optional steven_protocol::types::ParticleData);
    impl_serialize!(optional steven_protocol::types::VillagerData);
    impl_serialize!(steven_protocol::protocol::packet::PlayerInfoData);
    impl_serialize!(steven_protocol::types::Metadata);
    impl_serialize!(steven_protocol::protocol::packet::EntityEquipments);
    impl_serialize!(steven_shared::Position);
    impl_serialize!(std::string::String);

    #[cfg(feature = "serde_json")]
    impl_serialize!(serde_json::Value);

    use steven_protocol::protocol::LenPrefixedBytes;
    impl_serialize!(LenPrefixedBytes, steven_protocol::protocol::Lengthable);
    use steven_protocol::protocol::LenPrefixed;
    impl_serialize!(LenPrefixed, steven_protocol::protocol::Lengthable, steven_protocol::protocol::Serializable);
    use steven_protocol::protocol::{FixedPoint12,FixedPoint5};
    impl_serialize!(FixedPoint12, steven_protocol::protocol::Serializable);
    impl_serialize!(FixedPoint5, steven_protocol::protocol::Serializable);
    impl_serialize!(Vec<u8>);

}

