#[derive(Debug)]
struct ByteIter<'remainder> {
    remainder: &'remainder [u8],
}

impl<'remainder> ByteIter<'remainder> {
    fn next(&mut self) -> Option<&'remainder u8> {
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
    let mut bytes = ByteIter { remainder: b"1234" };
    dbg!(&bytes);
    let byte_1 = bytes.next();
    dbg!(&byte_1);
    let byte_2 = bytes.next();
    dbg!(&byte_2);
    std::mem::drop(bytes);
    if byte_1 == byte_2 {
        println!("bytes 1 = 2");
    } else {
        println!("bytes1 =! bytes 2");
    }
}
