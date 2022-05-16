// Fetching employees from datasource (DB, API, etc...)
const employees = [
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

// With functional approach, calculate the total salary 
function calculateSalary(employees) {
    return employees
        .map(e => e.salary)
        .reduce((p, s) => p + s, 0)
}
console.log(calculateSalary(employees)) // 34 + 13 + 20 = 67

// Next, business requires a new functionality, so we can wrote
function calculateSalaryForYoungEmployees(employees) {
    return employees
        .filter(e => e.age < 35)
        .map(e => e.salary)
        .reduce((p, s) => p + s, 0)
}
console.log(calculateSalaryForYoungEmployees(employees)) // 13 + 20 = 33

// But in a better (functional) way
function calculateSalary2(employees, filterFunction) {
    return employees
        .filter(filterFunction)
        .map(e => e.salary)
        .reduce((p, s) => p + s, 0)
}
const YOUNG_EMPLOYEE_THRESHOLD = 35
function isYoung(employee) {
    return employee.age < YOUNG_EMPLOYEE_THRESHOLD
}

console.log(calculateSalary2(employees, isYoung)) // 13 + 20 = 33
