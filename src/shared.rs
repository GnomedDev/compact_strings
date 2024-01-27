use crate::metadata::Metadata;

pub(crate) fn index_bytestr<'a>(
    data: &'a [u8],
    meta: &[Metadata],
    index: usize,
) -> Option<&'a [u8]> {
    let (start, len) = meta.get(index)?.as_tuple();
    if cfg!(feature = "no_unsafe") {
        data.get(start..start + len)
    } else {
        unsafe { Some(data.get_unchecked(start..start + len)) }
    }
}

#[cfg(not(feature = "no_unsafe"))]
pub(crate) unsafe fn index_bytestr_unchecked<'a>(
    data: &'a [u8],
    meta: &[Metadata],
    index: usize,
) -> &'a [u8] {
    let (start, len) = meta.get_unchecked(index).as_tuple();
    data.get_unchecked(start..start + len)
}

/// Iterator over bytestrings in a [`CompactBytestrings`] or [`FixedBytestrings`]
///
/// # Examples
/// ```
/// # use compact_strings::CompactBytestrings;
/// let mut cmpbytes = CompactBytestrings::new();
/// cmpbytes.push(b"One");
/// cmpbytes.push(b"Two");
/// cmpbytes.push(b"Three");
///
/// let mut iter = cmpbytes.into_iter();
/// assert_eq!(iter.next(), Some(b"One".as_slice()));
/// assert_eq!(iter.next(), Some(b"Two".as_slice()));
/// assert_eq!(iter.next(), Some(b"Three".as_slice()));
/// assert_eq!(iter.next(), None);
/// ```
#[must_use = "Iterators are lazy and do nothing unless consumed"]
pub struct ByteStringIter<'a> {
    data: &'a [u8],
    iter: core::slice::Iter<'a, Metadata>,
}

impl<'a> ByteStringIter<'a> {
    pub(crate) fn new(data: &'a [u8], meta: &'a [Metadata]) -> Self {
        Self {
            data,
            iter: meta.iter(),
        }
    }
}

impl<'a> Iterator for ByteStringIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let (start, len) = self.iter.next()?.as_tuple();

        if cfg!(feature = "no_unsafe") {
            self.data.get(start..start + len)
        } else {
            unsafe { Some(self.data.get_unchecked(start..start + len)) }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for ByteStringIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (start, len) = self.iter.next_back()?.as_tuple();

        if cfg!(feature = "no_unsafe") {
            self.data.get(start..start + len)
        } else {
            unsafe { Some(self.data.get_unchecked(start..start + len)) }
        }
    }
}

impl ExactSizeIterator for ByteStringIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}
