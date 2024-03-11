trait Supertrait {
    fn super_method(&mut self);
}

trait Subtrait: Supertrait {
    fn sub_method(&mut self);
}

struct CallSuperFromSub;

impl Supertrait for CallSuperFromSub {
    fn super_method(&mut self) {
        println!("(impl Supertrait)in super(CallSuperFromSub impl)");
    }
}

impl Subtrait for CallSuperFromSub {
    fn sub_method(&mut self) {
        println!("(impl Subtrait) in sub(CallSuperFromSub impl)");
        self.super_method();
    }
}

struct CallSubFromSuper;

impl Supertrait for CallSubFromSuper {
    fn super_method(&mut self) {
        println!(
            "(CallSubFromSuper) in super --> Supertrait for CallSubFromSuper(impl super_method"
        );
        self.sub_method();
    }
}

impl Subtrait for CallSubFromSuper {
    fn sub_method(&mut self) {
        println!("(CallSubFromSuper) in sub --> Subtrait for CallSubFromSuper(impl sub_method");
    }
}

struct CallEachOther(bool);

impl Supertrait for CallEachOther {
    fn super_method(&mut self) {
        println!("in super -- > Supertrait for CallEachOther(impl super_method");
        if self.0 {
            self.0 = false;
            self.sub_method();
        }
    }
}

impl Subtrait for CallEachOther {
    fn sub_method(&mut self) {
        println!("in sub --> Subtrait for CallEachOther(impl sub_method");
        if self.0 {
            self.0 = false;
            self.super_method();
        }
    }
}

fn main() {
    CallSuperFromSub.super_method(); // prints "in super"
    CallSuperFromSub.sub_method(); // prints "in sub", "in super"

    CallSubFromSuper.super_method(); // prints "in super" , "in sub"
    CallSubFromSuper.sub_method(); // prints "in sub"

    CallEachOther(true).super_method(); // prints "in super", "in sub"
    CallEachOther(true).sub_method(); // prints "in sub", "in super"
}
