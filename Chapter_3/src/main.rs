fn main() {
    /* Statements and Expressions:

    EXPRESSIONS

    Unlike statements, expressions evaluate to value and return it.

    (Corollary - function definitions are statements, yet function calls are 
    typically expressions)

    The math operation e.g. `5 + 6` is an expression, as it evaluates to `11`.

    Consider the block below... 
    */

    let result = expression_x();

    println!("The value of x is: {result}");
}

fn expression_x() -> i32 {
    {
        let x = 3;  //statement, doesn't evaluate, needs semi-colon
        x + 1       //note: No semi-colon here, this is fragment is an expression!
    }               //commenting out the assignment to y makes this a statement
                    //so I should not need a semi-colon on line 19

    //This expression compiles, and returns `4`.    
}
