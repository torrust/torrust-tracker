use std::io;

pub struct StackVec<'a, T: 'a> {
    data: &'a mut [T],
    length: usize,
}

impl<'a, T> StackVec<'a, T> {
    pub fn from(data: &mut [T]) -> StackVec<T> {
        StackVec { data, length: 0 }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[0..self.length]
    }
}

impl<'a, T> Extend<T> for StackVec<'a, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.data[self.length] = item;
            self.length += 1;
        }
    }
}

impl<'a> io::Write for StackVec<'a, u8> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() > (self.data.len() - self.length) {
            // not enough space on buffer.
            return Err(io::Error::from(io::ErrorKind::WriteZero));
        }
        let writable = &mut self.data[self.length..][0..buf.len()];
        writable.copy_from_slice(buf);
        self.length += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_write() {
        use std::io::Write;

        let mut buf = [0u8; 200];
        {
            let mut vec = StackVec::from(&mut buf);
            assert!(vec.write("Hello World!".as_bytes()).is_ok());
        }
        assert_eq!(buf[1] as char, 'e');
    }
}
