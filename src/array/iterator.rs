use super::Array;

/// An iterator that iterators on any [`Array`] type.
pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.array.len() {
            let item = self.array.get(self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array: &'a A) -> Self {
        Self { array, pos: 0 }
    }
}
