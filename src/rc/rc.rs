use core::ops::Deref;

use crate::alloc::{free, malloc};

pub struct Rc<T> {
    inner: *mut RcInner<T>,
}

struct RcInner<T> {
    strong: usize,
    content: T,
}

impl<T> Rc<T> {
    pub fn new(content: T) -> Self {
        let inner = malloc(1);
        unsafe { *inner = RcInner { strong: 1, content } };

        Self { inner }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe { &mut *self.inner }.strong += 1;
        Self { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe { &mut *self.inner }.strong -= 1;
        if unsafe { &*self.inner }.strong == 0 {
            free(self.inner);
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner).content }
    }
}
