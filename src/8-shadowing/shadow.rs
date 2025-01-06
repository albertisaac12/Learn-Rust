fn main(){
    let x = 5;
    {
        let x = 6;
        println!(" Inner X is : {x}");
    }
    
    let x =12; // shadowing
    println!(" Outer X after Shadowing is : {x}");

    let mut spaces = "   ";
    spaces = spaces.len();  // this will result in a error 
   // NOTE: the mut key word when used, the variable associated cant be used again for overshadowing
}