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
}

/* 
 
  each branch is evaluated top to bottom and the 1st matching condition
  executes, in this case "else if number % 3 == 0 ". N.B., had that
  branch not been there "else if number % 2 == 0" would have executed

 */