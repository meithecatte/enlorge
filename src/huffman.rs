#[derive(Clone, Debug)]
pub struct Huffman {
    order: Vec<u16>,
    counts: [u16; 16],
}

pub trait BitStream {
    fn get_bit(&mut self) -> bool;
}

impl Huffman {
    pub fn new(lengths: &[u8]) -> Self {
        let mut counts = [0_u16; 16];
        for &len in lengths {
            counts[len as usize] += 1;
        }

        let mut order = vec![0_u16; lengths.len() - counts[0] as usize];

        let mut pos = 0;
        let mut positions = [0; 16];
        for (i, count) in counts.iter().enumerate().skip(1) {
            positions[i] = pos;
            pos += count;
        }

        for (sym, &len) in lengths.iter().enumerate() {
            if len != 0 {
                let len = len as usize;
                order[positions[len] as usize] = sym as u16;
                positions[len] += 1;
            }
        }

        Huffman {
            order,
            counts,
        }
    }

    pub fn decode(&self, bits: &mut impl BitStream) -> u16 {
        let mut acc = 0_u16;
        let mut have_bits = 0;
        let mut pos = 0;
        loop {
            acc <<= 1;
            acc |= bits.get_bit() as u16;
            have_bits += 1;

            if acc < self.counts[have_bits] {
                return self.order[(pos + acc) as usize];
            } else {
                pos += self.counts[have_bits];
                acc -= self.counts[have_bits];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl BitStream for Vec<u8> {
        fn get_bit(&mut self) -> bool {
            self.pop().unwrap() != 0
        }
    }

    impl Huffman {
        fn check(&self, expect: u16, mut bits: Vec<u8>) {
            assert_eq!(self.decode(&mut bits), expect);
            assert!(bits.is_empty());
        }
    }

    #[test]
    fn example1() {
        let code = Huffman::new(&[2, 1, 3, 3]);
        code.check(0, vec![0, 1]);
        code.check(1, vec![0]);
        code.check(2, vec![0, 1, 1]);
        code.check(3, vec![1, 1, 1]);
    }

    #[test]
    fn example2() {
        let code = Huffman::new(&[3, 3, 3, 3, 3, 2, 4, 4]);
        code.check(0, vec![0, 1, 0]);
        code.check(1, vec![1, 1, 0]);
        code.check(2, vec![0, 0, 1]);
        code.check(3, vec![1, 0, 1]);
        code.check(4, vec![0, 1, 1]);
        code.check(5, vec![0, 0]);
        code.check(6, vec![0, 1, 1, 1]);
        code.check(7, vec![1, 1, 1, 1]);
    }
}
