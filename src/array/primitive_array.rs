//! Primitive array and array builders.
//!
//! This module implements array for primitive types, like `i32` and `f32`.

use bitvec::prelude::BitVec;

use super::{Array, ArrayBuilder, ArrayIterator};
use crate::scalar::{Scalar, ScalarRef};

/// A type that is primitive, such as `i32` and `i64`.
pub trait PrimitiveType: Scalar + Default {}

pub type I32Array = PrimitiveArray<i32>;
pub type F32Array = PrimitiveArray<f32>;

impl PrimitiveType for i32 {}
impl PrimitiveType for f32 {}

/// An [`Array`] that stores [`PrimitiveType`] items.
///
/// This array contains two parts: the value of each item, and the null bitmap of each item.
/// For example, if we create an [`Array`] of `[Some(1), None, Some(2)]`, it will be stored as
/// follows:
///
/// ```plain
/// data: [1, 0, 2]
/// bitmap: [true, false, true]
/// ```
///
/// We store the bitmap apart from data, so as to reduce memory footprint compared with
/// `Vec<Option<T>>`.
pub struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,

    bitmap: BitVec,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = Self>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = Self>,
    for<'a> T: Scalar<RefType<'a> = T>,
{
    type OwnedItem = T;
    /// For `PrimitiveType`, we can always get the value from the array with little overhead.
    /// Therefore, we do not use the `'a` lifetime here, and simply copy the value to the user when
    /// calling `get`.
    type RefItem<'a> = T;

    type Builder = PrimitiveArrayBuilder<T>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.bitmap[idx] {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = PrimitiveArray<T>>,
    for<'a> T: ScalarRef<'a, ScalarType = T, ArrayType = PrimitiveArray<T>>,
    for<'a> T: Scalar<RefType<'a> = T>,
{
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<T>) {
        match value {
            Some(v) => {
                self.data.push(v);
                self.bitmap.push(true);
            }
            None => {
                self.data.push(T::default());
                self.bitmap.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        PrimitiveArray {
            data: self.data,
            bitmap: self.bitmap,
        }
    }
}
