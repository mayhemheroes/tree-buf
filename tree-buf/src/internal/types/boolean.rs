use crate::internal::encodings::packed_bool::*;
use crate::prelude::*;
use std::vec::IntoIter;

#[cfg(feature = "write")]
impl<'a> Writable<'a> for bool {
    type WriterArray = Vec<bool>;
    #[inline]
    fn write_root<'b: 'a>(&'b self, _stream: &mut impl WriterStream) -> RootTypeId {
        if *self {
            RootTypeId::True
        } else {
            RootTypeId::False
        }
    }
}

#[cfg(feature = "read")]
impl Readable for bool {
    type ReaderArray = IntoIter<bool>;
    fn read(sticks: DynRootBranch<'_>, _options: &impl DecodeOptions) -> ReadResult<Self> {
        profile!("Readable::read");
        match sticks {
            DynRootBranch::Boolean(v) => Ok(v),
            _ => Err(ReadError::SchemaMismatch),
        }
    }
}

#[cfg(feature = "write")]
impl<'a> WriterArray<'a> for Vec<bool> {
    type Write = bool;
    fn buffer<'b: 'a>(&mut self, value: &'b Self::Write) {
        self.push(*value);
    }
    fn flush(self, stream: &mut impl WriterStream) -> ArrayTypeId {
        profile!("flush");
        stream.write_with_len(|stream| encode_packed_bool(&self, stream.bytes()));
        ArrayTypeId::Boolean
    }
}

#[cfg(feature = "read")]
impl InfallibleReaderArray for IntoIter<bool> {
    type Read = bool;
    
    fn new_infallible(sticks: DynArrayBranch<'_>, _options: &impl DecodeOptions) -> ReadResult<Self> {
        profile!("ReaderArray::new");

        match sticks {
            DynArrayBranch::Boolean(bytes) => {
                let v = decode_packed_bool(&bytes);
                Ok(v.into_iter())
            }
            _ => Err(ReadError::SchemaMismatch),
        }
    }
    fn read_next_infallible(&mut self) -> Self::Read {
        self.next().unwrap_or_default()
    }
}
