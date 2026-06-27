fn main() {

    //a further shadowing example

    let mut spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    //Does not compile... the MUT keyword will prevent type switching...
}
