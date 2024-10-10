//! String array and array builders.
//!
//! This module implements array for `String`. `String` is different from other types in the
//! following ways:
//!
//! * It is of variable length, and its storage layout is different from others.
//! * You can only get an `&str` from a `StringArray` (instead of `&String`).

use bitvec::prelude::BitVec;

use super::{Array, ArrayBuilder, ArrayIterator};

/// An [`Array`] that stores `String` items.
pub struct StringArray {
    data: Vec<u8>,
    offsets: Vec<usize>,
    bitmap: BitVec,
}

impl Array for StringArray {
    type OwnedItem = String;
    type RefItem<'a> = &'a str;

    type Builder = StringArrayBuilder;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.bitmap[idx] {
            let start = self.offsets[idx];
            let end = self.offsets[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[start..end]) })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.bitmap.len()
    }

    fn is_empty(&self) -> bool {
        self.bitmap.is_empty()
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

pub struct StringArrayBuilder {
    data: Vec<u8>,
    offsets: Vec<usize>,
    bitmap: BitVec,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            data: Vec::with_capacity(capacity),
            offsets: offsets,
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<&str>) {
        self.bitmap.push(value.is_some());
        if let Some(s) = value {
            self.data.extend_from_slice(s.as_bytes());
        }
        self.offsets.push(self.data.len());
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            data: self.data,
            offsets: self.offsets,
            bitmap: self.bitmap,
        }
    }
}
