// Rust doens't have the "null" / "undefined" variable
// (if you working in a safe mode). So, how to handle the
// absence of a value?
// The anwser is: with Option<T>
// Option<T> (it is an enumeration) allows you to define
// a variable that can only have 2 values:
// - None: means no value is stored
// - Some(T): means that the variable stores a value

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
