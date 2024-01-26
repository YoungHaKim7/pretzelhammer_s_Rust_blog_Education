use std::cell::Ref;

// elided
type T1 = Box<dyn Trait>;

// expanded, Box<T> has no lifetime bound on T, so ifeered as 'static
type T2 = Box<dyn Trait + 'static>;

// elided
impl dyn Trait {}

// expanded, &'a T requires T: 'a, so inferred as 'a
impl dyn Trait + 'static {}
trait Trait {
    // add code here
}

// elided
type T3<'a> = &'a dyn Trait;

// expanded, &'a T requires T: 'a, so inferred as 'a
type T4<'a> = &'a (dyn Trait + 'a);

// elided
type T5<'a> = Ref<'a, dyn Trait>;

// expanded, Ref<'a,T> requires T: 'a, so inferred as 'a
type T6<'a> = Ref<'a, dyn Trait + 'a>;

trait GenericTrait<'a>: 'a {}

// elided
type T7<'a> = Box<dyn GenericTrait<'a>>;

// expanded
type T8<'a> = Box<dyn GenericTrait<'a> + 'a>;

// elided
impl<'a> dyn GenericTrait<'a> {}

// expanded
impl<'a> dyn GenericTrait<'a> + 'a {}

fn main() {
    println!("Hello, world!");
}
