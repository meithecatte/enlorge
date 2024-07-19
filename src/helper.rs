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
