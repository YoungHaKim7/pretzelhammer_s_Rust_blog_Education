fn main() {
    let mut vec: Vec<()> = Vec::with_capacity(0);

    vec.push(()); // len++
    vec.push(());
    vec.push(());
    vec.push(());
    vec.push(());
    dbg!(vec.len());
}
