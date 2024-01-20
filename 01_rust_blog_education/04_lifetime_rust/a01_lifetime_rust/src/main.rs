// // eided
// fn print(s: &str);

// // expanded
// fn print01<'a>(s: &'a str);

// // elided
// fn trim(s: &str) -> &str;

// // expanded
// fn trim01<'a>(s: &'a str) -> &'a str;

// // illegal, can't determine output lifetime, no inputs
// fn get_str() -> &str;

// explicit options include
// fn get_str<'a>() -> &'a str; // generic version

// fn get_str() -> &'static str; // 'astatic version

// illegal, can't determine output lifetime, multile inputs
// fn overlap(s: &str, t: &str) -> &str;
// explicit (but still partially elided ) options include
// fn overlap<'a>(s: &'a str, t: &str) -> &'a str; // output can't outlive s
// fn overlap<'a>(s: str, t: &'a &str) -> &'a str; // output can't outlive t
// fn overlap<'a>(s: &'a str, t: &'a str) -> &'a str; // output can't outlive s & t
// fn overlap(s: &str, t: &str) -> &'static str; // output can outlive s & t
// fn overlap<'a>(s: &str, t: &str) -> &'a str; // no relationship between input & output lifetimes

// // expanded
// fn overlap<'a, 'b>(s: &'a str, t: &'b str) -> &'a str;
// fn overlap<'a, 'b>(s: &'a str, t: &'b str) -> &'b str;
// fn overlap<'a>(s: &'a str, t: &'a str) -> &'a str;
// fn overlap<'a, 'b>(s: &'a str, t: &'b str) -> &'static str;
// fn overlap<'a, 'b, 'c>(s: &'a str, t: &'b str) -> &'c str;

// // elided
// fn compare(&self, s: &str) -> &str;

// // expanded
// fn compare<'a, 'b>(&'a self, &'b str) -> &'a str;

fn main() {
    println!("Hello, world!");
}
