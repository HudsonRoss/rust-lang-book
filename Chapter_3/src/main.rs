fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1 //I left the semicolon off because this is an expression.
    }
}
/*
    works, but error prone because you are forced to track the index.
*/