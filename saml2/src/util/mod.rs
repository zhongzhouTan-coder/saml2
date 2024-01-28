use std::io::Read;

pub struct InputStream {
    buf: Vec<u8>,
    pos: usize,
}

impl InputStream {
    pub fn new(bytes: Vec<u8>) -> InputStream {
        InputStream { buf: bytes, pos: 0 }
    }

    #[inline(always)]
    pub fn reach_end(&self) -> bool {
        self.pos >= self.buf.len()
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 || self.reach_end() {
            return std::io::Result::Ok(0);
        }
        let (buf_len, pos_end) = (buf.len(), self.buf.len());
        let r_len = match self.pos + buf_len <= pos_end {
            true => buf_len,
            false => pos_end - self.pos,
        };
        for i in 0..r_len {
            buf[i] = self.buf[self.pos + i];
        }
        self.pos += r_len;
        std::io::Result::Ok(r_len)
    }
}
