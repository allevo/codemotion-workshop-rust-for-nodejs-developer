// This is an example of rust code

use std::{sync::Arc, thread};

// Define a structure called MyStruct
#[derive(Debug)]
struct MyStruct {
    field: usize,
    _unused_field: usize,
}

// This is how rust allow you to declare methods
impl MyStruct {
    // Commonly, the constructor is called new
    // Self means, literally, MyStruct
    fn new(field: usize) -> Self {
        Self {
            field: field,
            _unused_field: 0,
        }
    }
}

// Now we define 3 method that accept *self*
// but in 3 different way
// - MyStruct / mut MyStruct: consume the variable allowing the mutation
// - &mut MyStruct: capture a reference (a pointer) of an instance of MyStruct allowing the mutation
// - &MyStruct: capture a reference (a pointer) of an instance of MyStruct disallowing the mutation

fn consume(mut s: MyStruct) {
    // the following line is possible because this method keeps
    // an ownership of the instance of `MyStruct`.
    // This implies we are able to change the variable
    s.field += 1;
    println!("self: {:?}", s);
}

fn take_mut(s: &mut MyStruct) {
    // the following line is possible because this method keeps
    // a mutable reference to `MyStruct`.
    // This implies we are able to change the variable
    s.field += 1;
    println!("self: {:?}", s);
}

fn take_shared_ref(s: &MyStruct) {
    // the following line is *not* possible because this method keeps
    // a shared reference to `MyStruct`.
    // This implies we are *not* able to change the variable
    // Uncomment the next line to see what happen
    // s.field += 1;
    // Error:
    // cannot assign to `s.field`, which is behind a `&` reference
    // `s` is a `&` reference, so the data it refers to cannot be written
    println!("self: {:?}", s);
}

fn main() {
    println!("\nmy_struct_1:");
    let my_struct_1 = MyStruct::new(1);
    consume(my_struct_1);
    // We are not able to reuse `my_struct_1` after the line above
    // Try to comment the next line
    // println!("{:?}", my_struct_1);

    println!("\nmy_struct_2:");
    let mut my_struct_2 = MyStruct::new(2);
    // We are able to call multiple times the `take_mut` method
    // because `take_mut` doesn't consume `my_struct_2` instance
    take_mut(&mut my_struct_2);
    take_mut(&mut my_struct_2);
    take_mut(&mut my_struct_2);

    println!("\nmy_struct_3:");
    let my_struct_3 = MyStruct::new(3);
    // We are able to call multiple times the `take_shared_ref` method
    // because `take_shared_ref` doesn't consume `my_struct_2` instance
    take_shared_ref(&my_struct_3);
    take_shared_ref(&my_struct_3);
    take_shared_ref(&my_struct_3);

    // So, what differentiate mutable reference from shared reference?
    // I cannot have 2 mutable reference at the same time
    // In another words, I'm not able to access (in mutable way)
    // through 2 pointers to *the same allocation*
    // Instead, using shared reference we are allowed to keep multiple "pointers"
    // to the same allocation

    // Example: 2 threads reference to the same allocation and invoke `take_shared_ref`
    println!("\nmy_struct_4:");
    let my_struct_4 = Arc::new(MyStruct::new(4));
    let my_struct_4_1 = my_struct_4.clone();
    let my_struct_4_2 = my_struct_4.clone();
    println!("my_struct_4_1: {:p}", my_struct_4_1);
    println!("my_struct_4_2: {:p}", my_struct_4_2);

    let p1 = thread::spawn(move || {
        take_shared_ref(&my_struct_4_1);
    });
    let p2 = thread::spawn(move || {
        take_shared_ref(&my_struct_4_2);
    });

    p1.join().unwrap();
    p2.join().unwrap();

    // Try to change `take_shared_ref` to `take_mut`: you are not allowed to do that!
    // If you really want it, you need to use (explicitly) a mutex or similar
    // In order to handle correctly the concurrent writes

    // So, in rust we can declare if a method can change (or not) a parameter!
    // We will see how to implement the JS counterpart in a few moments
}
