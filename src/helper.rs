use bytes::Buf;
use std::ffi::CString;

fn strlen(data: &[u8]) -> Option<usize> {
    data.iter().position(|&c| c == 0)
}

pub trait BufExt: Buf {
    fn get_cstring(&mut self) -> CString {
        let chunk = self.chunk();
        if let Some(len) = strlen(chunk) {
            let s = CString::new(&chunk[0..len]).unwrap();
            self.advance(len + 1);
            return s;
        }

        let mut string = chunk.to_vec();
        loop {
            let chunk = self.chunk();
            if let Some(len) = strlen(chunk) {
                string.extend_from_slice(&chunk[0..len]);
                self.advance(len + 1);
                return CString::new(string).unwrap();
            } else {
                string.extend_from_slice(chunk);
                self.advance(chunk.len());
            }
        }
    }
}

impl<T: Buf> BufExt for T {}

pub struct BitReader<B: Buf> {
    bytes: B,
    leftover: u8,
    // Always between 0 and 7 bits
    count: u8,
}

impl<B: Buf> BitReader<B> {
    pub fn new(bytes: B) -> Self {
        Self {
            bytes,
            leftover: 0,
            count: 0,
        }
    }

    pub fn get_bits(&mut self, n: u8) -> u32 {
        // If self.count turns out to be smaller than n, we need to be able
        // to fit a whole byte into the "shift register".
        assert!(n <= 25);

        let mut bits = self.leftover as u32;
        while self.count < n {
            bits |= (self.bytes.get_u8() as u32) << self.count;
            self.count += 8;
        }

        // At this point, self.count is at most n + 7

        self.count -= n;
        self.leftover = (bits >> n) as u8;
        bits & ((1 << n) - 1)
    }

    pub fn get_bit(&mut self) -> bool {
        self.get_bits(1) != 0
    }
}
