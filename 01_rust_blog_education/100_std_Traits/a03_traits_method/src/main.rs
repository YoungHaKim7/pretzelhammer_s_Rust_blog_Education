trait Trait {
    // methods
    fn takes_self(self);
    fn takes_immut_self(&self);
    fn takes_mut_self(&mut self);

    // above methods desugared
    // fn takes_self(self: Self);
    // fn takes_immut_self(self: &Self);
    // fn takes_mut_self(self: &mut Self);
}

trait ToString {
    fn to_string(&self) -> String;
}

fn main() {
    let five = 5.to_string();
    dbg!(five);
}
