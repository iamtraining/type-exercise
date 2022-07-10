use super::iterator::ArrayIterator;
use super::models::{Array, ArrayBuilder};

use bitvec::prelude::BitVec;

// тип алиас для PrimitiveArray
trait Primitive: Default + Send + Sync + Copy + 'static {}

// структура для трейта
struct PrimitiveArray<T: Primitive> {
    data: Vec<T>,
    bitmap: BitVec,
}

// трейт
impl<P: Primitive> Array for PrimitiveArray<P> {
    // ссылка на возвращаемый элемент
    type RefItem<'a> = P;

    // билдер для примитивных типов
    type Builder = PrimitiveArrayBuilder<P>;

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

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

// структура билдера для примитивных типов
struct PrimitiveArrayBuilder<P: Primitive> {
    data: Vec<P>,
    bitmap: BitVec,
}

// реализация трейта билдера примитивных типов
impl<P: Primitive> ArrayBuilder for PrimitiveArrayBuilder<P> {
    type Array = PrimitiveArray<P>;

    fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            bitmap: BitVec::with_capacity(cap),
        }
    }

    fn finish(self) -> Self::Array {
        PrimitiveArray {
            data: self.data,
            bitmap: self.bitmap,
        }
    }

    fn push(&mut self, value: Option<P>) {
        match value {
            Some(v) => {
                self.data.push(v);
                self.bitmap.push(true);
            }
            None => {
                self.data.push(P::default());
                self.bitmap.push(false);
            }
        }
    }
}
