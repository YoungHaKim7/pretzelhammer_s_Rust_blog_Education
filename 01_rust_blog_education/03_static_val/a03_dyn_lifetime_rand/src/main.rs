use rand;

fn drop_static<T: 'static>(t: T) {
    std::mem::drop(t);
}

fn main() {
    let mut strings: Vec<String> = Vec::new();
    for _ in 0..10 {
        if rand::random() {
            let string = rand::random::<u64>().to_string();
            strings.push(string);
        }
    }
    println!("strings : {:?}", &strings);

    for mut string in strings {
        string.push_str(" a mutation");
        println!("strings : {:?}", &string);
        drop_static(string);
    }
}
