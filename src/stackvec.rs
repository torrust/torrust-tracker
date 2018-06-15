use std::io;

pub struct StackVec<'a, T: 'a> {
    data: &'a mut [T],
    length: usize,
}

impl<'a, T> StackVec<'a, T> {
    pub fn from(data: &mut [T]) -> StackVec<T> {
        StackVec{
            data,
            length: 0,
        }
    }
}

impl<'a, T> StackVec<'a, T> {
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<'a> io::Write for StackVec<'a, u8> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() > (self.data.len() - self.length) {
            // not enough space on buffer.
            return Err(io::Error::from(io::ErrorKind::WriteZero));
        }
        let mut writable = &mut self.data[self.length..][0..buf.len()];
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

    fn add_values<T: io::Write>(vec: &mut T) {
        for i in 0..BUF_LEN {
            assert!(vec.write(&[((i % 256) & 0xff) as u8]).is_ok());
        }
    }

    use test::Bencher;

    const BUF_LEN : usize = 1024 * 1024 * 1; // 10MB

    #[bench]
    fn vec_stack(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut buff = [0u8; BUF_LEN];
            let mut v = StackVec::from(&mut buff);
            add_values(&mut v);
        });
    }

    #[bench]
    fn vec_heap(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut v = Vec::new();

            add_values(&mut v);
        });
    }

    #[bench]
    fn vec_heap_cap(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut v = Vec::with_capacity(BUF_LEN);
            add_values(&mut v);
        });
    }
}