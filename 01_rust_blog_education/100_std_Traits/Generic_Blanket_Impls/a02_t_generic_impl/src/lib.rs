use std::{convert::TryInto, fmt::Debug, ops::Rem};

trait Even {
    fn is_even(self) -> bool;
}

impl<T> Even for T
where
    T: Rem<Output = T> + PartialEq<T> + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: Debug,
{
    fn is_even(self) -> bool {
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
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
