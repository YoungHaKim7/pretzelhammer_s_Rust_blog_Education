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
    let mut bytes = ByteIter {
        remainder: b"132355",
    };
    let byte_1 = bytes.next();
    let byte_2 = bytes.next();
    std::mem::drop(bytes); //drop
                           // let byte_3 = bytes.next(); // error code
    if byte_1 == byte_2 {
        println!("byte1 == byte2");
    } else {
        println!("byte1 != byte2");
    }
}
