use alloc::boxed::Box;

use small_fixed_array::{FixedArray, ValidLength};

use crate::shared::ByteStringIter;
#[allow(clippy::wildcard_imports)]
use crate::{metadata::Metadata, shared::*, CompactBytestrings};

pub struct FixedBytestrings<CountT: ValidLength = u32> {
    data: FixedArray<u8>,
    meta: FixedArray<Metadata, CountT>,
}

impl<CountT: ValidLength> FixedBytestrings<CountT> {
    /// Returns a reference to the bytestring stored in the [`FixedBytestrings`] at that position.
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        index_bytestr(&self.data, &self.meta, index)
    }

    /// Returns a reference to the bytestring stored in the [`FixedBytestrings`] at that position, without
    /// doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    #[must_use]
    #[cfg(not(feature = "no_unsafe"))]
    pub unsafe fn get_unchecked(&self, index: usize) -> &[u8] {
        index_bytestr_unchecked(&self.data, &self.meta, index)
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter(&self) -> ByteStringIter<'_> {
        ByteStringIter::new(&self.data, &self.meta)
    }
}

impl<'a, CountT: ValidLength> IntoIterator for &'a FixedBytestrings<CountT> {
    type Item = &'a [u8];
    type IntoIter = ByteStringIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
enum TryFromCompactBytesErrorRepr<CountT: ValidLength> {
    Data(<FixedArray<u8, CountT> as TryFrom<Box<[u8]>>>::Error),
    Meta(<FixedArray<Metadata, CountT> as TryFrom<Box<[Metadata]>>>::Error),
}

#[derive(Debug)]
pub struct TryFromCompactBytesError<CountT: ValidLength>(TryFromCompactBytesErrorRepr<CountT>);

#[cfg(feature = "std")]
impl<CountT: ValidLength> std::error::Error for TryFromCompactBytesError<CountT> {}

impl<CountT: ValidLength> core::fmt::Display for TryFromCompactBytesError<CountT> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            TryFromCompactBytesErrorRepr::Data(err) => err.fmt(f),
            TryFromCompactBytesErrorRepr::Meta(err) => err.fmt(f),
        }
    }
}

impl<CountT: ValidLength> TryFrom<CompactBytestrings> for FixedBytestrings<CountT> {
    type Error = TryFromCompactBytesError<CountT>;

    fn try_from(value: CompactBytestrings) -> Result<Self, Self::Error> {
        let data = value.data.into_boxed_slice().try_into();
        let meta = value.meta.into_boxed_slice().try_into();

        Ok(Self {
            data: data
                .map_err(TryFromCompactBytesErrorRepr::Data)
                .map_err(TryFromCompactBytesError)?,
            meta: meta
                .map_err(TryFromCompactBytesErrorRepr::Meta)
                .map_err(TryFromCompactBytesError)?,
        })
    }
}
