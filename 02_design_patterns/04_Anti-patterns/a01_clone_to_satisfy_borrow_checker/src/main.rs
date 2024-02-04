fn main() {
    let mut x = 5;

    let y = &mut (x.clone());

    println!("{x}");
    *y += 1;
    dbg!(y);
}
