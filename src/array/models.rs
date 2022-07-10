use super::iterator::ArrayIterator;

pub trait ArrayBuilder {
    type Array: Array<Builder = Self>;

    fn with_capacity(cap: usize) -> Self;

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

    fn finish(self) -> Self::Array;
}

pub trait Array: Send + Sync + Sized + 'static {
    type RefItem<'a>: Copy;

    type Builder: ArrayBuilder<Array = Self>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}
