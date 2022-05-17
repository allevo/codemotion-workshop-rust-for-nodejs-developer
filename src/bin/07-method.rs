use std::{sync::Arc, thread};

#[derive(Debug)]
struct MyStruct {
    field: usize,
    _unused_field: usize,
}

impl MyStruct {
    fn new(field: usize) -> Self {
        Self {
            field: field,
            _unused_field: 0,
        }
    }

    fn consume_self(mut self) {
        // the following line is possible because this method keeps
        // an ownership of the instance of `Self`.
        // This implies we are able to change the variable
        self.field += 1;
        println!("self: {:?}", self);
    }

    fn take_mut(&mut self) {
        // the following line is possible because this method keeps
        // a mutable reference to `Self`.
        // This implies we are able to change the variable
        self.field += 1;
        println!("self: {:?}", self);
    }

    fn take_shared_ref(&self) {
        // the following line is *not* possible because this method keeps
        // a shared reference to `Self`.
        // This implies we are *not* able to change the variable
        // Uncomment the next line to see what happen
        // self.field += 1;
        println!("self: {:?}", self);
    }
}

fn main() {
    println!("\nmy_struct_1:");
    let my_struct_1 = MyStruct::new(1);
    my_struct_1.consume_self();
    // We are not able to reuse `my_struct_1` after the line above
    // Try to comment the next line
    // println!("{:?}", my_struct_1);

    println!("\nmy_struct_2:");
    let mut my_struct_2 = MyStruct::new(2);
    // We are able to call multiple times the `take_mut` method
    // because `take_mut` doesn't consume `my_struct_2` instance
    my_struct_2.take_mut();
    my_struct_2.take_mut();
    my_struct_2.take_mut();

    println!("\nmy_struct_3:");
    let my_struct_3 = MyStruct::new(3);
    // We are able to call multiple times the `take_shared_ref` method
    // because `take_shared_ref` doesn't consume `my_struct_2` instance
    my_struct_3.take_shared_ref();
    my_struct_3.take_shared_ref();
    my_struct_3.take_shared_ref();

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
        my_struct_4_1.take_shared_ref();
    });
    let p2 = thread::spawn(move || {
        my_struct_4_2.take_shared_ref();
    });

    p1.join().unwrap();
    p2.join().unwrap();

    // Try to change `take_shared_ref` to `take_mut`: you are not allowed to do that!
    // If you really want it, you need to use (explicitly) a mutex or similar
    // In order to handle correctly the concurrent writes
}
