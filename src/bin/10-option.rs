#[derive(Debug)]
struct Employee {
    id: String,
    name: String,
    salary: usize,
    age: usize,
}

fn main() {
    let employees: Vec<Employee> = fetch_employees();
    // We are not sure to find a "John" inside the vector
    // so, `find` method returns an `Option` of `Employee`
    let john: Option<Employee> = employees
        .into_iter()
        .find(|e: &Employee| e.name == "John".to_owned());
    // John is found, so the Option is fulfilled with Some(employee)
    println!("john: {:?}", john);

    let employees: Vec<Employee> = fetch_employees();
    let foo: Option<Employee> = employees
        .into_iter()
        .find(|e: &Employee| e.name == "Foo".to_owned());
    // Foo is not found, so the Option is not fulfilled and is equal to None
    println!("foo: {:?}", foo);
}

fn fetch_employees() -> Vec<Employee> {
    vec![
        Employee {
            id: "the-user-id-1".to_owned(),
            name: "John".to_owned(),
            age: 45,
            salary: 34,
        },
        Employee {
            id: "the-user-id-2".to_owned(),
            name: "Tom".to_owned(),
            age: 22,
            salary: 13,
        },
        Employee {
            id: "the-user-id-3".to_owned(),
            name: "Ross".to_owned(),
            age: 30,
            salary: 20,
        },
    ]
}
