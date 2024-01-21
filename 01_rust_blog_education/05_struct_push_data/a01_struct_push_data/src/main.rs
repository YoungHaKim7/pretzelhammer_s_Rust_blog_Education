// Implementing the trait `std::fmt::Debug`
// to allow a struct instance to be printed
#[derive(Debug)]
struct Employee {
    firstname: String,
    lastname: String,
}

fn main() {
    let mut emp1 = Employee {
        firstname: String::from("NoName"),
        lastname: String::from("NoName"),
    };

    emp1.firstname = String::from("Tattva");
    emp1.lastname = String::from("Hegde");

    let mut emp_db: Vec<Employee> = vec![];
    emp_db.push(emp1);

    let x = &emp_db[0];

    println!("Hello, world! {:#?}", x);
}
