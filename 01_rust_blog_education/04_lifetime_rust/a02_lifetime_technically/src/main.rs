struct ByteIter<'b> {
    remainder: &'b [u8],
}

impl<'b> ByteIter<'b> {
    fn next(&mut self) -> Option<&u8> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}

fn main() {
    let mut bytes = ByteIter { remainder: b"1" };
    assert_eq!(Some(&b'1'), bytes.next());
    assert_eq!(None, bytes.next());
}
