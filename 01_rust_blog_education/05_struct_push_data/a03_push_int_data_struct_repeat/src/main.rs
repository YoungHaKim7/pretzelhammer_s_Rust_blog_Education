// Implementing the trait `std::fmt::Debug`
// to allow a struct instance to be printed
#[derive(Debug)]
struct Employee {
    data: i32,
}

impl Employee {
    fn add(&self, value: i32) -> Self {
        // Take value to add as argument
        Self {
            data: self.data + value,
        }
    }
}

fn main() {
    let mut emp1 = Employee { data: 100 };

    // emp1.data += 200;

    let mut emp_db: Vec<Employee> = vec![];
    emp_db.push(emp1.add(200));
    emp_db.push(emp1.add(200));

    let x = &emp_db[0];

    println!("Hello, world! {:#?}", x);
}
