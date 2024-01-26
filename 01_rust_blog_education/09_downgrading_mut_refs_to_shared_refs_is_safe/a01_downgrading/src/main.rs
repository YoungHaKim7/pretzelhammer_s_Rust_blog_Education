fn takes_shard_ref(n: &i32) {}
fn main() {
    let mut a = 10;
    // takes_shard_ref(&mut a);
    // takes_shard_ref(&*(&mut a)); // above line desugared
    let b: &i32 = &*(&mut a); // re-borrowed as immutable
    let c: &i32 = &a;
    dbg!(b, c); // error
}
