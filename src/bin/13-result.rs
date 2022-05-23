// Error handling is a worse part of a program.
// Rust tries to make it as easy as possible
// introducing the concept of Result
// Result is an enumeration that can have only 2 values:
// - Ok(T): means that your variable correctly contains a value
// - Err(E): means that your variable contains an error
// Let see...

#[derive(Debug)]
struct Employee {
    id: String,
    name: String,
    salary: usize,
    age: usize,
}

// The arithmetic mean is equal to the sum of all elements divided by the count
fn calculate_salary_mean(employees: Vec<Employee>) -> usize {
    let number: usize = employees.len();
    let total: usize = employees.into_iter().map(|e: Employee| e.salary).sum();

    total / number
}

fn main() {
    let employees: Vec<Employee> = fetch_employees();

    let mean = calculate_salary_mean(employees);
    println!("mean: {:?}", mean);

    // But what happen if the vector is empty?
    // are we able to calculate also??
    let mean = calculate_salary_mean(vec![]);
    println!("mean: {:?}", mean);
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
