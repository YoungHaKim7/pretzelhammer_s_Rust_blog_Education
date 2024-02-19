fn example<T>(t: &[T]) -> Vec<T> {
    unimplemented!() // ! coered to Vec<T>
}

fn example2() -> i32 {
    match "1243".parse::<i32>() {
        Ok(num) => num,
        Err(_) => unreachable!(),
    }
}

fn example3(some_condition: bool) -> &'static str {
    if !some_condition {
        panic!("oops");
    } else {
        "str"
    }
}

fn example4() -> i32 {
    let x: String = {
        return 1243; // ! coerced to String
    };
}

fn example5(nums: &[i32]) -> Vec<i32> {
    let mut filtered = Vec::new();
    for num in nums {
        filtered.push(if *num < 0 {
            break; // ! coerced to i32
        } else if *num % 2 == 0 {
            *num
        } else {
            continue;
        });
    }
    filtered
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // example();
    example2();
    dbg!(example3(true));
    print_type_of(&(example4()));
    dbg!(example4());
    dbg!(example5(&[10]));
    dbg!(example5(&[11]));
    dbg!(example5(&[8]));
    dbg!(example5(&[-8]));
}
