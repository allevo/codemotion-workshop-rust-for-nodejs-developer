struct Employee {
    id: String,
    name: String,
    salary: usize,
    age: usize,
}

fn main() {
    let employees: Vec<Employee> = fetch_employees();
    let salary: usize = employees.into_iter().map(|e: Employee| e.salary).sum();

    println!("Salary: {}", salary);

    let employees: Vec<Employee> = fetch_employees();
    let salary: usize = employees
        .into_iter()
        .filter(|e: &Employee| {
            // Uncomment follow line
            // e.salary += 5;
            // `e` is a `&` reference, so the data it refers to cannot be written
            e.age < 35
        })
        .map(|e: Employee| e.salary)
        .sum();

    println!("Salary: {}", salary);
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
