fn main() {
    // Create new scope though "{"
    {
        let foo = 5;

        // this closes the scope, deallocating all the variable (stack and heap)
        // The compiler puts the deallocation here
    }
    // try to decomment the following line
    // println!("{}", foo);
    // The error is:
    // cannot find value `foo` in this scope
    // not found in this scope

    let foo: String = "the foo string".to_string();
    capture_the_variable(foo);
    // Try to decomment the following line
    // println!("{}", foo);
    // The error is:
    // borrow of moved value: `foo`
    // value borrowed here after move
}

//
fn capture_the_variable(foo: String) {
    // Do somthing with *foo*
}
