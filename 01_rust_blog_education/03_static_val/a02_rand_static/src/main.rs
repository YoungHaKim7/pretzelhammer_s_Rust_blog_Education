use rand;

fn rand_str_generator() -> &'static str {
    let rand_string = rand::random::<u64>().to_string();
    Box::leak(rand_string.into_boxed_str())
}
fn main() {
    println!("random str: {}", rand_str_generator());
}
