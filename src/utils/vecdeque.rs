use core::mem::MaybeUninit;

use crate::error::Error;

pub struct VecDeque<T, const N: usize> {
    buff: [MaybeUninit<T>; N],
    start: usize,
    len: usize,
}

impl<T, const N: usize> VecDeque<T, N> {
    pub fn new() -> Self {
        Self {
            buff: [const { MaybeUninit::uninit() }; N],
            start: 0,
            len: 0,
        }
    }

    pub fn push_back(&mut self, val: T) -> Result<usize, Error> {
        if self.len == N {
            return Err(Error::VecFull(N));
        }

        let end = self.end();

        self.buff[end] = MaybeUninit::new(val);
        self.len += 1;
        Ok(self.len)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let last = (self.start + self.len + N - 1) % N;
        let t = core::mem::replace(&mut self.buff[last], MaybeUninit::uninit());

        self.len -= 1;

        if self.len == 0 {
            self.start = 0;
        }

        // SAFETY: `last` b/w (start..start+len), i.e within initialised range.
        Some(unsafe { t.assume_init() })
    }

    pub fn push_front(&mut self, val: T) -> Result<usize, Error> {
        if self.len == N {
            return Err(Error::VecFull(N));
        }

        let vacant = (self.start + N - 1) % N;
        self.buff[vacant] = MaybeUninit::new(val);
        self.start = vacant;
        self.len += 1;

        Ok(self.len)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let t = core::mem::replace(&mut self.buff[self.start], MaybeUninit::uninit());
        self.start = (self.start + 1) % N;

        self.len -= 1;

        if self.len == 0 {
            self.start = 0;
        }

        // SAFETY: `self.start` will point to an index which had an initialised value.
        Some(unsafe { t.assume_init() })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        // Go from start..end and run destructors for all of them.

        for i in 0..self.len() {
            // SAFETY: The values from start..end are initialised as per push/pop logic defined above.
            unsafe {
                self.buff[(self.start + i) % N].assume_init_drop();
            }
        }

        (self.start, self.len) = (0, 0);
    }

    fn end(&self) -> usize {
        (self.start + self.len) % N
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        // SAFETY: Index is within the initialised range
        Some(unsafe { self.buff[(self.start + index) % N].assume_init_ref() })
    }

    pub fn iter(&self) -> VecDequeIterator<T, N> {
        VecDequeIterator {
            deque: self,
            offset: 0,
        }
    }
}

impl<T, const N: usize> Drop for VecDeque<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct VecDequeIterator<'a, T, const N: usize> {
    deque: &'a VecDeque<T, N>,
    offset: usize,
}

impl<'a, T, const N: usize> Iterator for VecDequeIterator<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.deque.get(self.offset)?;
        self.offset += 1;
        Some(ret)
    }
}
