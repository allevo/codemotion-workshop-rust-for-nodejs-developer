// But commonly we split the code into multiple files

// File 1
let employees = [
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
console.log(calculateSalary2(employees, isYoung))
