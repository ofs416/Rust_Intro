fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // C++ s1 would make a copy, in rust it would move to the function scope and would need to be returned
    // Like c++ we can instead reference it like &s1

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");
    change(&mut s2); // This indicates that the reference is mutable
    println!("{s2}");

    let r1 = &s1;
    let r2 = &s1; // ok because immutable

    let r3 = &mut s2;
    // let r4 = &mut s2;  not ok because cannot borrow mutable more than once before usage

    println!("{r1}, {r2}");

    // Data Races:
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    let r5 = &mut s2;
    println!("{r5}");
    let r6 = &s2;
    println!("{r6}");
    // println!("{r5}"); This creates a data race

    {
        let mut s = String::from("hello");

        let r7 = &s; // no problem
        let r8 = &s; // no problem
        let r9 = &mut s; // BIG PROBLEM

        // println!("{r8}"); r8 expects the value to not change, therefore wont compile
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // Variables r1 and r2 will not be used after this point.

        let r3 = &mut s; // no problem
        println!("{r3}")
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
