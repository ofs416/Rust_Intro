fn main() {
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.
    //
    // Function bodies are made up of a series of statements optionally ending in an expression

    // A new scope is an expression
    let y = {
        let x = 3;
        x + 1
    };

    // functions calls are expressions, they have a resultant value, which isn't always returned to
    // the code that call it
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    // function definitions are statements
    fn plus_two(x: i32) -> i32 {
        return x + 2;
    }

    let y2 = plus_two(y);
    println!("y + 2 = {y2}")
}
