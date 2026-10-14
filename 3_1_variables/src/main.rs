fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //declared in any scope
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing works similar to python

    x = x * 2;
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Value of spaces: {spaces}");

    let x = 5;
    let y = 2;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of x is: {x}");
}

// so let redeclares variables, needed if variable is not mutable
// this is needed even when shadowing if the type is changing, otherwise it isnt needed.
