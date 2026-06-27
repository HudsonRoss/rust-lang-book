fn main() {

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 5;

    let y = THREE_HOURS_IN_SECONDS;

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");    

    println!("There are {y} seconds in 3 hours.");
}
