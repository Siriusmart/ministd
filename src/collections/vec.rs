use core::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr::{self, null_mut},
    slice::{self, SliceIndex},
};

use crate::alloc::{free, malloc};

pub struct Vec<T> {
    first: *mut T,
    capacity: usize,
    length: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            first: null_mut(),
            capacity: 0,
            length: 0,
        }
    }

    pub fn insert(&mut self, i: usize, item: T) {
        assert!(
            i <= self.length,
            "index out of bounds: inserting to index {i} for vector of length {}",
            self.length
        );

        match self.length {
            0 => {
                self.first = malloc(1);

                assert!(
                    self.first != null_mut(),
                    "insert failed: malloc returned null"
                );

                self.capacity = 1;
            }
            len if len == self.capacity => {
                self.capacity <<= 1;
                let new_first: *mut T = malloc(self.capacity);

                assert!(
                    self.first != null_mut(),
                    "insert failed: malloc returned null"
                );

                for i in 0..self.length {
                    unsafe { ptr::copy_nonoverlapping(self.first.add(i), new_first.add(i), 1) }
                }

                free(self.first);
                self.first = new_first;
            }
            _ => {}
        }

        unsafe { *self.first.add(self.length) = item };
        self.length += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        assert!(
            i < self.length,
            "index out of bounds: removing from index {i} for vector of length {}",
            self.length
        );

        let out = unsafe { ptr::read(self.first.add(i)) };

        for j in i..self.length - 1 {
            unsafe { ptr::copy_nonoverlapping(self.first.add(j + 1), self.first.add(j), 1) };
        }

        self.length -= 1;

        out
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.length {
            Some(unsafe { &*self.first.add(i) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < self.length {
            Some(unsafe { &mut *self.first.add(i) })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, item: T) {
        self.insert(self.length, item);
    }

    pub fn pop(&mut self) -> T {
        self.remove(self.length - 1)
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for Vec<T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        let s = unsafe { slice::from_raw_parts(self.first, self.length) };
        &s[index]
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for Vec<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let s = unsafe { slice::from_raw_parts_mut(self.first, self.length) };
        &mut s[index]
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self[..]
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self[..]
    }
}

impl<T: Ord> Vec<T> {
    fn qsort(v: &mut [T]) {
        if v.is_empty() {
            return;
        }

        fn partition<T: Ord>(v: &mut [T]) -> usize {
            let mut j = 0;

            for i in 0..v.len() - 1 {
                if v[i] <= v[v.len() - 1] {
                    v.swap(i, j);
                    j += 1;
                }
            }

            v.swap(j, v.len() - 1);
            j
        }

        let q = partition(v);
        Self::qsort(&mut v[..q]);
        Self::qsort(&mut v[q + 1..]);
    }

    pub fn sort(&mut self) {
        Self::qsort(self);
    }
}

impl<T: Debug> Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (&**self).fmt(f)
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        free(self.first);
    }
}
