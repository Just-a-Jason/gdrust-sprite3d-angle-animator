// use godot::{builtin::Array, meta::ArrayElement};

// pub struct ArrayIterator<T: ArrayElement> {
//     inner: Array<T>,
//     index: usize
// }

// impl<T: ArrayElement> ArrayIterator<T> {
//     pub fn new(array: Array<T>) -> Self {
//         ArrayIterator { inner: array, index: 0 }
//     }

//     pub fn get(&self, index:usize) -> Option<T> {
//         self.inner.get(index)
//     }
// }

// impl<T: ArrayElement> Iterator for ArrayIterator<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let item = self.inner.get(self.index);
//         self.index += 1;
//         item
//     }
// }
