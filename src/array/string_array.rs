use bitvec::prelude::BitVec;

use super::models::{Array, ArrayBuilder};

// структура для трейта
pub struct StringArray {
    // сглаженные данные строки
    data: Vec<u8>,
    // смещения каждой строки в плоском массиве
    offsets: Vec<usize>,
    // пустой bitmap этого массива
    bitmap: BitVec,
}

// накидал get
impl StringArray {
    fn get(&self, idx: usize) -> Option<&str> {
        if self.bitmap[idx] {
            let range = self.offsets[idx]..self.offsets[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
        } else {
            None
        }
    }
}

// трейт Array
impl Array for StringArray {
    type RefItem<'a> = &'a str;

    type Builder = StringArrayBuilder;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.get(idx)
    }

    fn len(&self) -> usize {
        self.bitmap.len()
    }
}

// структура билдера для строк
pub struct StringArrayBuilder {
    data: Vec<u8>,
    offsets: Vec<usize>,
    bitmap: BitVec,
}

// реализация трейта билдера
impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(cap: usize) -> Self {
        let mut offsets = Vec::with_capacity(cap + 1);
        offsets.push(0);
        Self {
            data: Vec::with_capacity(cap),
            offsets,
            bitmap: BitVec::with_capacity(cap),
        }
    }

    fn finish(self) -> Self::Array {
        StringArray {
            data: self.data,
            offsets: self.offsets,
            bitmap: self.bitmap,
        }
    }

    fn push(&mut self, value: Option<&str>) {
        match value {
            Some(v) => {
                self.data.extend(v.as_bytes());
                self.bitmap.push(true);
                self.offsets.push(self.data.len());
            }
            None => {
                self.offsets.push(self.data.len());
                self.bitmap.push(false);
            }
        }
    }
}
