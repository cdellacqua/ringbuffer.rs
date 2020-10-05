use core::ops::{Index, IndexMut};

use crate::ringbuffer_trait::RingBuffer;

extern crate alloc;
// We need vecs so depend on alloc
use alloc::vec::Vec;
use core::iter::FromIterator;

/// The AllocRingBuffer is a RingBuffer which is based on a Vec. This means it allocates at runtime
/// on the heap, and therefore needs the [`alloc`] crate. This struct and therefore the dependency on
/// alloc can be disabled by disabling the `alloc` (default) feature.
///
/// # Example
/// ```
/// use ringbuffer::{AllocRingBuffer, RingBuffer};
///
/// let mut buffer = AllocRingBuffer::with_capacity(2);
///
/// // First entry of the buffer is now 5.
/// buffer.push(5);
///
/// // The last item we pushed is 5
/// assert_eq!(buffer.get(-1), Some(&5));
///
/// // Second entry is now 42.
/// buffer.push(42);
///
/// assert_eq!(buffer.peek(), Some(&5));
/// assert!(buffer.is_full());
///
/// // Because capacity is reached the next push will be the first item of the buffer.
/// buffer.push(1);
/// assert_eq!(buffer.to_vec(), vec![42, 1]);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct AllocRingBuffer<T> {
    buf: Vec<T>,
    cap: usize,
    index: usize,
}

/// The capacity of a RingBuffer created by new or default (`1024`).
pub const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

impl<T: 'static + Default> RingBuffer<T> for AllocRingBuffer<T> {
    #[inline]
    fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    fn clear(&mut self) {
        self.buf.clear();
        self.index = 0;
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.cap
    }

    fn push(&mut self, e: T) {
        if self.buf.len() < self.capacity() {
            self.buf.push(e);
        } else {
            self.buf[self.index] = e;
        }

        self.index = (self.index + 1) % self.capacity()
    }

    impl_ringbuffer!(buf, index);
}

impl<T> AllocRingBuffer<T> {
    /// Creates a RingBuffer with a certain capacity. This capacity is fixed.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "Capacity must be greater than zero");

        Self {
            buf: Vec::with_capacity(cap),
            cap,
            index: 0,
        }
    }

    /// Creates a RingBuffer with a capacity of [RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<RB: 'static + Default> FromIterator<RB> for AllocRingBuffer<RB> {
    fn from_iter<T: IntoIterator<Item = RB>>(iter: T) -> Self {
        let mut res = Self::default();
        for i in iter {
            res.push(i)
        }

        res
    }
}

impl<T> Default for AllocRingBuffer<T> {
    /// Creates a buffer with a capacity of [crate::RINGBUFFER_DEFAULT_CAPACITY].
    #[inline]
    fn default() -> Self {
        let cap = RINGBUFFER_DEFAULT_CAPACITY;
        Self {
            buf: Vec::with_capacity(cap),
            cap,
            index: 0,
        }
    }
}

impl<T: 'static + Default> Index<isize> for AllocRingBuffer<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: 'static + Default> IndexMut<isize> for AllocRingBuffer<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use crate::{AllocRingBuffer, RINGBUFFER_DEFAULT_CAPACITY, RingBuffer};
    use super::alloc::vec::Vec;

    #[test]
    fn test_default() {
        let b: AllocRingBuffer<u32> = AllocRingBuffer::default();
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.capacity());
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.buf.capacity());
        assert_eq!(b.cap, b.capacity());
        assert_eq!(b.buf.len(), b.len());
        assert_eq!(0, b.index);
        assert!(b.is_empty());
        assert!(b.buf.is_empty());
        assert_eq!(0, b.iter().count());
        assert_eq!(
            Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY),
            b.buf
        );
        assert_eq!(
            Vec::<u32>::with_capacity(RINGBUFFER_DEFAULT_CAPACITY),
            b.to_vec()
        );
    }

    #[test]
    fn test_default_capacity_constant() {
        // This is to prevent accidentally changing it.
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, 1024)
    }

    #[test]
    #[should_panic]
    fn test_index_zero_length() {
        let b = AllocRingBuffer::<i32>::with_capacity(2);
        let _ = b[2];
    }
}
