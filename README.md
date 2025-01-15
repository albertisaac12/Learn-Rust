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

Rust's `char` type is `four bytes` in size and represents a Unicode Scalar Value

Mutable `Tuples` are also possible in Rust

The `floating-point` type by default is `f64` but `f32` is also possible, while `f64 is double precision` `f32 is single precision`

**If you add a semicolon `;` to the end of an expression, you turn it into a statement, and it will then not return a value.**

## STACK HEAP

**All data stored on the stack must have a known, fixed size.**
**Data with an unknown size at compile time or a size that might change must be stored on the heap instead.**
**The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.**
**This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating)**

## NOTE

**Each value in Rust has an owner.**
**There can only be one owner at a time.**
**When the owner goes out of scope, the value will be dropped.**

---

# NOTE: **READ THE ./src/Udemy/ownership.rs for the ownership**

---

## Slices

Slices in rust let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

Slices essentially point to data.

They are passed by reference to functions i.e borrowing. You can use them to fetch portions of data and customize what you what to slice.

# NOTE FROM RUST BOOK

**If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. We would say that s1 was moved into s2.**

**In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.**

**The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.**

**Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.**
