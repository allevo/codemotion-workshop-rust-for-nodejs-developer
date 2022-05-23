// We saw before some rules about the *ownership*.
// Leveraging on that rules, we can say more:
// in fact, Rust knows at **compile time** when a variable
// will be deallocated (ad freed)
// Rust (and not you) puts a *delete* when a variable goes
// out of the scope
// Let see...

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
    // *capture_the_variable* captures the variable
    // The rust compile (borrow checker) ensures that
    // no other piece of code access to *foo* variable.
    // Because of this property, at the end of *capture_the_variable*
    // function, Rust can put a *free* invocation for *foo* variable
    capture_the_variable(foo);
    // Try to decomment the following line
    // println!("{}", foo);
    // The error is:
    // borrow of moved value: `foo`
    // value borrowed here after move
}

//
fn capture_the_variable(foo: String) {
    
    // Do somothing with *foo*, or not, it is up to you!

} // <--- rust automatically deallocate *foo*
