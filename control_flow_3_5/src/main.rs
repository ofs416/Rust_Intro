fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Mix between python and c++

    let condition = true;

    let number = if condition { 5 } else { 6 }; // types have to be the same

    println!("The value of number is: {number}");

    // loops: controlled with breaks and continues just like python

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // While loops

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For loops

    for number in (1..11).step_by(2).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
