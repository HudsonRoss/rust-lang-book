fn main() {
    //Remember, in Rust, arrays must have a fixed length.

    let a = [1, 2, 3, 4, 5];

    println!("{}", a[2]);

    //To be more explicit about the type, do like the following:

    let b: [i32; 5] = [6, 7, 8, 9, 10]; // as you see, the type is i32, and it is 5 elements long

    println!("{}", b[4]);
}
