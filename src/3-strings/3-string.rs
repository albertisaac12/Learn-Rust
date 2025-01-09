fn main(){
    // String Literal (&str) or String slices found in the module std::str.
    // String Object (String)

    let greeting = "Hello world";

    /*
    
        String literals are static by default. This ensures that the string is valid for the entire duration of the program.
        String literals are immutable my   nature 

        In Rust, string literals are static by default. 
        This means that when you define a string literal, 
        it has a fixed, unchanging value that exists for the entire duration of the program. 
        Rust stores these string literals in a special memory area called the "data segment" (often referred to as read-only memory or static memory).

        You can explicitly declare the string as static
    
     */

    let bank2:& 'static str = "hi there";

    // string object

    let nothing = String::new();
    println!("{}",nothing.len());

    let great_movie = String::from("Helloooooo");
    println!("{}",great_movie.len());

    // String methods
    /*
        The Sting object are mutable
        new()
        to_string()
        len()
        from()
        push()
        replace()
        push_str()
        trim()
        split_withespace()
        split()
        as_str()
        chars()
     */

    let mut greeting = String::from("hey there, ");
    greeting.push_str("Hello");
    println!("{}",greeting);

    // convert a string literal into a string Object

    let randum_string:&str = "convert mee";
    // let randum_string = "convert mee".to_string();
    let fff = randum_string.to_string();
    println!("{}",fff);

    let mut bb = "pokemon".to_string();
    bb.push_str(" Gotta catch them all");

    println!("{}",bb);

}