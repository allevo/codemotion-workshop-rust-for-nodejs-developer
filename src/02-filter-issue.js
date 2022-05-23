// But commonly we split the code into multiple files:
// probably not the best structure of the code, but no one
// here is complaining because it's a workshop :)

// Supposing we split the code in the following way:
// File 1: fetchEmployees()
// File 2: calculateSalary2() -- the last version of the function we saw previously
// FIle 3: isYoung()
// Main: starting point of our application

// File 1
function fetchEmployees() {
    return [
        {
            id: 'the-user-id-1',
            name: 'John',
            age: 45,
            salary: 34
        },
        {
            id: 'the-user-id-2',
            name: 'Tom',
            age: 22,
            salary: 13
        },
        {
            id: 'the-user-id-3',
            name: 'Ross',
            age: 30,
            salary: 20
        }
    ]
}

// File 2
function calculateSalary2(employees, filterFunction) {
    return employees
        .filter(filterFunction)
        .map(e => e.salary)
        .reduce((p, s) => p + s, 0)
}

// File 3
const YOUNG_EMPLOYEE_THRESHOLD = 35
function isYoung(employee) {
    return employee.age < YOUNG_EMPLOYEE_THRESHOLD
}

// Main
console.log(calculateSalary2(fetchEmployees(), isYoung))

// Are we able to be sure that *isYoung* function is just a filtering function
// and doesn't change our object?
// are there any checks that save me to *not* write something like:
function isYoung2(employee) {
    if (employee.age < 20) {
        employee.salary *= 0.5
    }
    return employee.age < YOUNG_EMPLOYEE_THRESHOLD
}