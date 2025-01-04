fn main(){
    // String Literal (&str) or String slices found in the module std::str.
    // String Object (String)

    let greeting = "Hello world";

    /*
    
        String literals are static by default. This ensures that the string is valid for the entire duration of the program.
        String literals are immutable my nature 

        In Rust, string literals are static by default. 
        This means that when you define a string literal, 
        it has a fixed, unchanging value that exists for the entire duration of the program. 
        Rust stores these string literals in a special memory area called the "data segment" (often referred to as read-only memory or static memory).

        You can explicitly declare the string as static
    
     */

    let bank2:& 'static str = "hi there ";

}