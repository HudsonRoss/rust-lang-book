fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1; 

        if counter == 10 {
            break counter * 2 
            //think of this as 'counter * 2, just before you break'
        }
    };

    println!("The result is {result}");
}