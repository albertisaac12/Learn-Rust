# Learn-Rust

Rust Programming Lang

String literals are static by default. This ensures that the string is valid for the entire duration of the program.
String literals are immutable my nature

In Rust, string literals are static by default.
This means that when you define a string literal,
it has a fixed, unchanging value that exists for the entire duration of the program.
Rust stores these string literals in a special memory area called the "data segment" (often referred to as read-only memory or static memory).

You can explicitly declare the string as static

Static String declaration `let bank2:& 'static str = "hi there ";` this is for a string Literal

Match Operator

`let micro = "xc12";

    let body_part = match micro {
        "xc12" => {println!("Found  the match"); "Tummy Biome"},
        _=>"unknown"
    };

    println!("{}",body_part);`

// for loop

    `for a in 1..20{ // 20 is exculded [1,20)
        if a%2 == 0{
            continue; // sends the control back to the begging of the loop while moving on to the next iteration
        }
        println!("{}",a);
    }`

// an indefinite loop

    `let mut ad = 0;
    while ad<5{
        println!("{}",ad);
        ad+=1;
    }`

// The loop keyword

`let mut c=0;

    loop {
        c-=1;
        println!("{}",c);
        if c == -10 {
            break;
        }
    }`

`for _ in 0..5{
        aa();
    }`

// Building a function example

`fn aa() {
    println!("{}","HEY THERE");
}`

`fn bb() -> String{
    return "Bonjour, Je suis Abhi".to_string();
}`

`fn cc(blah:i32) -> i32{
    return blah-10;
}`

## Tuples

Tuples are compound data types.
Compound types can store values at the same time of different data types.

Tuples have a fixed length - once declared they cannot grow or shrink in size.
The tuple index start from 0.

decleration `let ff:(i8,f32,i32) = (2,2.2,100);`

### To print a Tuple we use :? `println!("{:?}",ff);` here ff is the tuple.

---

### `{:?} more specifically the :?` is known as the debug formatting the Debug Trait is for detailed and developer-focused output that shows more information about a type, often in a less polished way.

### `{}` is the Display Trait used for clean and human-readable output meant for end users. For example, printing a number or message.

---

## Arrays in Rust

An array consists of sequential memory blocks.

Arrays are static by default.

Arrays cannot be resized once initialized

Each memory block represents an array element.

Array elements are identified by the index.

---

Array element values can be updated or modified but cannot be deleted.

Length is known at the compile time Signature: [T; length].

### Even the arrays do not implement the Display Trait

`let arrd:[&str; 4] = ["ab","bc","cd","de"];
// let arrd = ["ab","bc","cd","de"];`

Both of the above are the same in the commented section the rust will auto assign the data type and length

Arrays can be made mutable with the `mut` keyword like any other mutable data types

### iter() Function iterates over the array values

// the iter function

`for val in c.iter(){
 println!("{}",val);
}
`

---
