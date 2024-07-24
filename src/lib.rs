pub struct StackVec<T: Copy, const S: usize> {
    buffer: [T; S],
    len: usize,
}

impl<T: Copy, const S: usize> StackVec<T, S> {
    pub const fn new() -> Self {
        Self {
            buffer: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }
    pub fn push(&mut self, value: T) {
        if self.len < S {
            self.buffer[self.len] = value;
            self.len += 1;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { *self.buffer.get_unchecked(self.len) })
        } else {
            None
        }
    }
    pub const fn len(&self) -> usize {
        self.len
    }
    pub const fn capacity(&self) -> usize {
        S
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            Some(unsafe { *self.buffer.get_unchecked(index) })
        } else {
            None
        }
    }
    pub fn rev(&mut self) {
        let n = self.len;
        let n2 = n >> 1;
        for i in 0..n2 {
            let temp = unsafe { *self.buffer.get_unchecked(i) };
            self.buffer[i] = unsafe { *self.buffer.get_unchecked(n - i - 1) };
            self.buffer[n - i - 1] = temp;
        }
    }
    pub fn split(&mut self, index: usize) -> Self {
        if index >= self.len {
            Self::new()
        } else {
            let mut other = Self::new();
            let mut l = 0;
            for i in index..self.len {
                unsafe {
                    *other.buffer.get_unchecked_mut(l) = *self.buffer.get_unchecked(i);
                }
                l += 1;
            }
            other.len = l;
            self.len = index;
            other
        }
    }
    pub fn join(&mut self, arr: Self) {
        arr.iter().for_each(|&x| self.push(x));
    }
    pub const fn iter(&self) -> StackVecIter<T, S> {
        StackVecIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct StackVecIter<'v, T: Copy, const S: usize> {
    vec: &'v StackVec<T, S>,
    index: usize,
}

impl<'v, T: Copy, const S: usize> Iterator for StackVecIter<'v, T, S> {
    type Item = &'v T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let ret = &self.vec.buffer[self.index];
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl<T: Copy, const S: usize> FromIterator<T> for StackVec<T, S> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut ret = Self::new();
        while let Some(x) = iter.next() {
            ret.push(x);
        }
        ret
    }
}

impl<T: Copy + PartialOrd, const S: usize> StackVec<T, S> {
    pub fn sort(&mut self) {
        if self.len > 1 {
            for i in 1..self.len {
                if unsafe { self.buffer.get_unchecked(i) }
                    > unsafe { self.buffer.get_unchecked((i - 1) >> 1) }
                {
                    let mut j = i;
                    while unsafe { self.buffer.get_unchecked(j) }
                        > unsafe { self.buffer.get_unchecked((j - 1) >> 1) }
                    {
                        (self.buffer[j], self.buffer[(j - 1) >> 1]) =
                            (self.buffer[(j - 1) >> 1], self.buffer[j]);
                        j = (j - 1) >> 1;
                        if j < 1 {
                            break;
                        }
                    }
                }
            }
            for i in (1..self.len).rev() {
                let temp = unsafe { *self.buffer.get_unchecked(i) };
                self.buffer[i] = unsafe { *self.buffer.get_unchecked(0) };
                self.buffer[0] = temp;
                let mut j = 0;
                loop {
                    let mut index = (j << 1) + 1;
                    if index < i - 1 {
                        if unsafe { self.buffer.get_unchecked(index) }
                            < unsafe { self.buffer.get_unchecked(index + 1) }
                        {
                            index += 1;
                        }
                    }
                    if index < i {
                        if unsafe { self.buffer.get_unchecked(j) }
                            < unsafe { self.buffer.get_unchecked(index) }
                        {
                            (self.buffer[j], self.buffer[index]) =
                                (self.buffer[index], self.buffer[j]);
                        }
                    }
                    j = index;
                    if index >= i {
                        break;
                    }
                }
            }
        }
    }
    pub fn sort_rev(&mut self) {
        if self.len > 1 {
            for i in 1..self.len {
                if unsafe { self.buffer.get_unchecked(i) }
                    < unsafe { self.buffer.get_unchecked((i - 1) >> 1) }
                {
                    let mut j = i;
                    while unsafe { self.buffer.get_unchecked(j) }
                        < unsafe { self.buffer.get_unchecked((j - 1) >> 1) }
                    {
                        (self.buffer[j], self.buffer[(j - 1) >> 1]) =
                            (self.buffer[(j - 1) >> 1], self.buffer[j]);
                        j = (j - 1) >> 1;
                        if j < 1 {
                            break;
                        }
                    }
                }
            }
            for i in (1..self.len).rev() {
                let temp = unsafe { *self.buffer.get_unchecked(i) };
                self.buffer[i] = unsafe { *self.buffer.get_unchecked(0) };
                self.buffer[0] = temp;
                let mut j = 0;
                loop {
                    let mut index = (j << 1) + 1;
                    if index < i - 1 {
                        if unsafe { self.buffer.get_unchecked(index) }
                            > unsafe { self.buffer.get_unchecked(index + 1) }
                        {
                            index += 1;
                        }
                    }
                    if index < i {
                        if unsafe { self.buffer.get_unchecked(j) }
                            > unsafe { self.buffer.get_unchecked(index) }
                        {
                            (self.buffer[j], self.buffer[index]) =
                                (self.buffer[index], self.buffer[j]);
                        }
                    }
                    j = index;
                    if index >= i {
                        break;
                    }
                }
            }
        }
    }
}

impl<T: Copy + PartialEq, const S: usize> StackVec<T, S> {
    pub fn have(&self, value: T) -> bool {
        if self.len > 0 {
            for i in 0..self.len {
                if unsafe { *self.buffer.get_unchecked(i) } == value {
                    return true;
                }
            }
        }
        false
    }
    pub fn distinct(&mut self) {
        if self.len > 1 && self.len < S {
            let mut ret = Self::new();
            for i in 0..self.len {
                if !ret.have(unsafe { *self.buffer.get_unchecked(i) }) {
                    ret.push(self.buffer[i]);
                }
            }
            *self = ret;
        }
    }
}

impl<T: Copy + std::fmt::Debug, const S: usize> std::fmt::Debug for StackVec<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_list();
        for i in 0..self.len {
            f.entry(&self.buffer[i]);
        }
        f.finish()
    }
}

impl<T: Copy, const S: usize, const Z: usize> From<[T; Z]> for StackVec<T, S> {
    fn from(value: [T; Z]) -> Self {
        if Z > S {
            let mut ret = Self::new();
            for i in 0..S {
                ret.push(value[i]);
            }
            ret
        } else {
            let mut ret = Self::new();
            value.iter().for_each(|&x| ret.push(x));
            ret
        }
    }
}

impl<T: Copy, const S: usize> From<Vec<T>> for StackVec<T, S> {
    fn from(value: Vec<T>) -> Self {
        if value.len() > S {
            let mut ret = Self::new();
            for i in 0..S {
                ret.push(value[i]);
            }
            ret
        } else {
            let mut ret = Self::new();
            value.iter().for_each(|&x| ret.push(x));
            ret
        }
    }
}

#[macro_export]
macro_rules! stackvec {
    () => {};
    ($n:literal) => {
        StackVec::<_, $n>::new()
    };
    ($n:literal $t:ty) => {
        StackVec::<$t, $n>::new()
    };
    ($x:expr; $n:expr) => {
        StackVec::<_, $n>::from([$x; $n])
    };
    ($n:literal = $($x:expr),+ $(,)?) => {{
        let mut ret = StackVec::<_, $n>::new();
        $(
            ret.push($x);
         )+
         ret
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_get() {
        let result = StackVec::<f32, 3>::new();
        assert_eq!(result.len(), 0);
        assert_eq!(result.capacity(), 3);
        assert_eq!(result.get(0), None);
    }

    #[test]
    fn from_push_pop_clear() {
        let mut result = StackVec::<f32, 5>::from([1.0, 2.0, 3.0]);
        assert_eq!(result.len(), 3);
        assert_eq!(result.get(3), None);
        result.push(6.6);
        assert_eq!(result.get(3), Some(6.6));
        assert_eq!(result.len(), 4);
        let temp = result.pop();
        assert_eq!(temp, Some(6.6));
        assert_eq!(result.len(), 3);
        result.clear();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn macro_iter_reverse_sort() {
        let mut result = stackvec!(10 = 1, 3, 2);
        assert_eq!(result.len(), 3);
        assert_eq!(result.capacity(), 10);
        let mut iter = result.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
        result.rev();
        let mut iter = result.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        result.sort();
        let mut iter = result.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        result.sort_rev();
        let mut iter = result.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn split_join() {
        let mut result = stackvec!(6;3);
        let other = result.split(1);
        assert_eq!(result.capacity(), 3);
        assert_eq!(other.capacity(), 3);
        assert_eq!(result.len(), 1);
        assert_eq!(other.len(), 2);
        result.join(other);
        assert_eq!(result.len(), 3);
    }
}
