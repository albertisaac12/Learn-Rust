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
