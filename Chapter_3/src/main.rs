fn main() {
    /* Statements and Expressions:

    EXPRESSIONS

    Unlike statements, expressions evaluate to value and return it.

    (Corollary - function definitions are statements, yet function calls are 
    typically expressions)

    The math operation e.g. `5 + 6` is an expression, as it evaluates to `11`.

    Consider the block below... 
    */

    let y = {
        let x = 3;  //statement, doesn't evaluate, needs semi-colon
        x + 1       //note: No semi-colon here, this is fragment is an expression!
    };              //this whole block does an assignment action, but returns
                    // nothing. Hence, it is a statement, ending in a semicolon
}

