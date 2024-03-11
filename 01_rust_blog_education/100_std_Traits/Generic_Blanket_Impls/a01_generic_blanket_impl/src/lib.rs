trait Even {
    fn is_even(self) -> bool;
}

impl Even for i8 {
    fn is_even(self) -> bool {
        self % 2_i8 == 0_i8
    }
}

impl Even for u8 {
    fn is_even(self) -> bool {
        self % 2_u8 == 0_u8
    }
}

impl Even for i16 {
    fn is_even(self) -> bool {
        self % 2_i16 == 0_i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_even_impl() {
        let my_val: i8 = 4;
        println!("my_val even? = {}", my_val.is_even());

        assert!(2_u8.is_even());

        assert!(4_i8.is_even());
        assert!(8_i16.is_even());
    }
}
