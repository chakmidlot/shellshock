use core::marker::PhantomData;
use core::mem::MaybeUninit;

pub struct Ring<const C: usize, T> {
    data: [MaybeUninit<T>; C],
    index: usize,
    capacity: usize,
    wrapped: bool,
}

impl<const C: usize, T: Copy> Ring<C, T> {
    pub(crate) fn new() -> Self {
        Self {
            data: [MaybeUninit::<T>::uninit(); C],
            index: 0,
            capacity: C,
            wrapped: false,
        }
    }

    pub fn add(&mut self, value: T) {
        self.data[*&self.index] = MaybeUninit::new(value);

        self.index += 1;

        if self.index == self.capacity {
            self.index = 0;
            self.wrapped = true;
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            data: &self.data as *const MaybeUninit<T>,
            index: (self.index + self.capacity - 1) % self.capacity,
            capacity: self.capacity,
            wrapped: self.wrapped,
            iterator_wrapped: false,
            iterator_index: (self.index + self.capacity - 1) % self.capacity,
            _phantom: PhantomData,
        }
    }
}

pub struct Iter<'a, T> {
    data: *const MaybeUninit<T>,
    index: usize,
    capacity: usize,
    wrapped: bool,
    iterator_index: usize,
    iterator_wrapped: bool,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if *&self.iterator_wrapped && (&self.iterator_index == &self.index) {
            return None;
        }

        if !&self.wrapped && (self.iterator_index == &self.capacity - 1) {
            return None;
        }

        let v: &T = unsafe { &*(self.data.add(self.iterator_index) as *const T) };

        self.iterator_index = (&self.iterator_index + &self.capacity - 1) % self.capacity;
        if self.iterator_index == self.capacity - 1 {
            self.iterator_wrapped = true
        }

        return Some(v);
    }
}
