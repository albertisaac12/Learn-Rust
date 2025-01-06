fn main(){
    /*
        What are Functions in RUST !!!
        1) self contained
    
    
     */

    for _ in 0..5{
        // aa();
        let ac =bb();
        println!("{}",ac);
        let bv = cc(0);
        println!("{}",bv);
    }

    println!("{}",sem_fn());
}

// Building a function example

fn aa() {
    println!("{}","HEY THERE");
}

fn bb() -> String{
    return "Bonjour, Je suis Abhi".to_string();
}

fn cc(blah:i32) -> i32{
    aa();
    return blah-10;
}

/*
    Difference between Expressions and Statements

    Expressions : Anything that evaluates to a return value 
    Statements : An instruction that performs some action and does not return any value.

    Example of a Statement : let y = 10; here y is being binded with the value 10

*/

// example of a statement 

fn sem_fn()-> i32{
    let y = {
        let x = 0;
        x+10
    };
    return y;
}

// here there is something important to understand that if a semicolon ; is not append at the end it becomes a expression and a value is returned automatically