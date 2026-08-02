fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5

    /* There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. 
    That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32. */
    
}
