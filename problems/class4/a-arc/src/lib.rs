#![allow(dead_code)]

//! Simple implementation of Arc
//!
//! No weak reference, strong reference only.
//!
//! ## Examples
//!
//! ```
//! use a_arc::Arc;
//!
//! let a = Arc::new(1);
//! let b = a.clone();
//! ```

use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;

struct ArcInner<T> {
    counter: AtomicUsize,
    data: T,
}

/// Simple Arc
pub struct Arc<T> {
    inner: NonNull<ArcInner<T>>,
    _marker: PhantomData<ArcInner<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    /// Creates new Arc
    pub fn new(_data: T) -> Arc<T> {
        todo!()
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::sync::RwLock;
    use std::thread;

    use super::*;

    #[test]
    fn simple_test() {
        let a = Arc::new(1);
        let b = a.clone();

        let t = thread::spawn(move || {
            assert_eq!(*b, 1);
        });

        assert_eq!(*a, 1);

        t.join().unwrap();
    }

    #[test]
    fn mut_test() {
        let a = Arc::new(RwLock::new(0));

        let t = (0..10)
            .map(|_| {
                let b = a.clone();
                thread::spawn(move || {
                    let mut w = b.write().unwrap();
                    *w += 1;
                })
            })
            .collect::<Vec<_>>();

        t.into_iter().for_each(|j| j.join().unwrap());
        assert_eq!(*a.read().unwrap(), 10);
    }
}
