struct ByteIter<'a> {
    remainder: &'a [u8],
}

impl<'a> ByteIter<'a> {
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
    let mut bytes = ByteIter {
        remainder: b"112355",
    };
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
    println!("{:?}", bytes.next());
}
