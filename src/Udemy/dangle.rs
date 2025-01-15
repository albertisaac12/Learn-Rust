fn main() {
    let reference_to_nothing = dangle(); // here we "do" return the reference of the s before the function ends, this is the issue because once the scope is done String is dropped freeing the memory
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
/*
    The issue at hand is quite simple the &s dose'nt transfer ownership rather its transferring the borrowed value 
    but we know that if the ownership is not moved before the scope terminates the variable will be dropped and memory will be freed
    Now in the above code the return value is basically pointing to a free memory that leads to errors

*/