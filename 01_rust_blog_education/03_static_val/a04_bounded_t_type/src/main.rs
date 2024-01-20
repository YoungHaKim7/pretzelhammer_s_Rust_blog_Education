// only takes ref types bounded by 'a
fn t_ref<'a, T: 'a>(t: &'a T) {}

// takes any types bounded by 'a
fn t_bound<'a, T: 'a>(t: T) {}

// owned type which contains a reference
struct Ref<'a, T: 'a>(&'a T);

fn main() {
    let string = String::from("string");

    t_bound(&string);
    t_bound(Ref(&string));
    t_bound(&Ref(&string));

    t_ref(&string);
    // t_ref(Ref(&string)); - expected ref, found struct
    t_ref(&Ref(&string));

    // string var is bounded by 'static which is bounded by 'a
    t_bound(string);
}
