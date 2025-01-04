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