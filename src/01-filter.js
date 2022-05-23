// Supposing we are fetching employees from datasource (DB, API, etc...)
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

// With functional approach, we can calculate the total salary 
function calculateSalary(employees) {
    return employees
        // mapping every employee into its salary
        .map(e => e.salary)
        // calculate the total salary as the sum of the all salaries
        .reduce((p, s) => p + s, 0)
}
console.log(calculateSalary(fetchEmployees())) // 34 + 13 + 20 = 67

// Next, business asked to us an enhancement: we need to
// calculate the total salary but only for "young" employee
// where "young" means less then 35
// So, we can wite the following function
function calculateSalaryForYoungEmployees(employees) {
    return employees
        // we filter out leveraging on the age
        .filter(e => e.age < 35)
        .map(e => e.salary)
        .reduce((p, s) => p + s, 0)
}
console.log(calculateSalaryForYoungEmployees(fetchEmployees())) // 13 + 20 = 33

// But, as developer, we care about the maintainability of our code
// so, using a functional approach, we can split the salary calculation
// from the filtering. In this way we generalize the solution and allow
// future changes 
function calculateSalary2(employees, filterFunction) {
    return employees
        .filter(filterFunction)
        .map(e => e.salary)
        .reduce((p, s) => p + s, 0)
}

// This is the injected filter function
const YOUNG_EMPLOYEE_THRESHOLD = 35
function isYoung(employee) {
    return employee.age < YOUNG_EMPLOYEE_THRESHOLD
}

// so we can combine in easy way:
console.log(calculateSalary2(fetchEmployees(), isYoung)) // 13 + 20 = 33
