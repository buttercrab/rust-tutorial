//! Simple implementation of Arc
//!
//! No weak reference, strong reference only.
//!
//! ## Examples
//!
//! ```
//! use a_arc_sol::Arc;
//!
//! let a = Arc::new(1);
//! let b = a.clone();
//! ```

use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    pub fn new(data: T) -> Arc<T> {
        let inner = Box::new(ArcInner {
            counter: AtomicUsize::new(1),
            data,
        });

        Arc {
            inner: NonNull::new(Box::into_raw(inner)).unwrap(),
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.inner.as_ref() };
        &inner.data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.counter.fetch_add(1, Ordering::Relaxed);

        if count >= isize::MAX as usize {
            std::process::abort();
        }

        Arc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        if inner.counter.fetch_sub(1, Ordering::Relaxed) != 1 {
            return;
        }

        atomic::fence(Ordering::Acquire);

        unsafe {
            drop(Box::from_raw(self.inner.as_ptr()));
        }
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
