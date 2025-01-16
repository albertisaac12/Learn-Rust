
fn main(){
    let blah = "yep".to_string();
    println!("{}",blah.len());
    let slice1 = &blah[0..];
    println!("{}",slice1);
    let slice2 = &blah[..3];
    println!("{}",slice2);
    let slice3 = &blah[1..2];
    println!("{}",slice3);

    let mut nums = [1,2,3,4,5];
    change(&mut nums);
    println!("{:?}",nums);

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    
    s.clear(); // this empties the String, making it equal to ""
    
    println!("{word}");
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn change(x: &mut [i32]){
    println!("{:?}",x);
    x[0] = 200;
    println!("{:?}",&x[1..3]);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/*
    When a variable goes out of scope, Rust calls a special function for us. 
    This function is called drop, and it’s where the author of String can put the code to return the memory. 
    Rust calls drop automatically at the closing curly bracket.

    Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. 
    If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.
*/

// references have no ownership only value which can be borrowed 