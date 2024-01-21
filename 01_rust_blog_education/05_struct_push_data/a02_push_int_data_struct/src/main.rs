// Implementing the trait `std::fmt::Debug`
// to allow a struct instance to be printed
#[derive(Debug)]
struct Employee {
    data: i32,
}

fn main() {
    let mut emp1 = Employee { data: 100 };

    emp1.data += 200;

    let mut emp_db: Vec<Employee> = vec![];
    emp_db.push(emp1);
    // emp_db.push(&emp1);
    // emp_db.push(&emp1);

    let x = &emp_db[0];

    println!("Hello, world! {:#?}", x);
}
