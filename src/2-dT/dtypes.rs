
// let is used to declare variables
fn main(){
    // let df = "blah blah blah"; // string type
    // let a = 10; // int type
    // let b = false; // bool
    // // println!(df,a,b);

    // println!("blah is : {} {} {}",df,a,b);

    /*
        macros are pre-processor directive.
        Macros are pre-processed which means that all the macros would be processed before your program compiles However, functions are not preprocessed but compiled
    
        format!
        print!
        println!
        eprint!
        eprintln!

        Rust checks the formatting correctness at compile time
    */

    /*
        {} will be replaced with arguments and will be stringified
    */

    println!("{u1}\n{u2}\n{u3}",u1="a",u2="b",u3="c");

    /*
        Integers are basically not the float numbers => whole numbers

        int(+ve and -ve values) and uint(+ve values only)

        the size of the integer can be set to the arch means that the size of the int will be that of the hardware architecture
    
     */

    let total:i32  = 4; // default will be i32
    let height:u32=41;
    let deduction:i32=2-200;

    // println!("The total is {c}",c=total+height+deduction); Gives a error because of type mismatch

    // range for i(n) is -2^(n-1) to 2^(n-1)-1

    // let f:u16 = 65535;
    // let a:u16= 65535+1;
    // let b=f;

    // println!("{}\n{}\n{}",f,a,b);

    //varaibles are immutable by default
    //variables are read only by default

    // we can apply thg mut keyword in order to make the variable mutable


    let mut mc = 100;
    mc= 200;

    println!("{}",mc);

    // constants in rust 

    const BC:i32 = 10;

    // overshadowing variables work for variables and will override the variable

    let mp = 10;
    let mp = 12;
    println!("mp: {} ",mp);

    // but doing the same with const will result in a error  



}