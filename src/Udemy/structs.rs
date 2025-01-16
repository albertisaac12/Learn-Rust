struct House {
    door:String,
    garden:String,
    property_value:u32
}

struct Triangle {
    base:u32,
    height:u32
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
/*
    Rust does include functionality to print out debugging information, 
    but we have to explicitly opt in to make that functionality available for our struct. To do that, 
    we add the outer attribute #[derive(Debug)] just before the struct definition

*/
#[derive(Debug)]
struct blah{
    blah:String
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
impl Triangle{
    fn area(&self)->u32{
        self.height*self.base/2
    }
}

fn main(){
    // println!("asdasdasd");

    // struct example

    let home1 = House {
        door:String::from("Blue"),
        garden:String::from("asdasda"),
        property_value:1000000
    };

    println!("{}",home1.door);


    let t1 = Triangle{
        base:10,
        height:30
    };

    let a1 = t1.area();
    println!("The are of t1 is {}",a1);


    // using the struct update syntax to create a new struct from an exsisting struct 

    let home2= House{
        door:String::from("black"),
        ..home1
    };
    println!("{}",home2.garden);

    let a1 = Color(1,2,3);
    let a2 = Point(4,5,6);

    println!("{}",a1.0);

    let ac = blah{
        blah:String::from("hello")
    };

    println!("{:?}",ac);
    println!("{ac:?}");
    println!("{:#?}",ac);
    println!("{ac:#?}");

    // dbg!(ac); // doing this will transfer the ownership so always prefer to use the println with or dbg with & i.e use values
    dbg!(&ac);
    // another thing to note 
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    /*
    We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, 
    he width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, 
    so we use a reference to rect1 in the next call.
    */
    
    dbg!(&rect1);
}

// example of method

/*
    essentially a struct is like an object that holds diffrent data types unlike tuple we do named declaration of these types inside struct
    Methods are functions which belong in the scope within a structure.

    You declare the methods with the impl keyword outside of the structure block.

    Important to note that the parameter of a method will be always self,
    which represents the calling instance of the structure

*/

/*

    Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable
    As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

*/

/*

    The ..home1 must come last to specify that any remaining fields should get their values from the corresponding fields in home1, 
    but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.

*/

/*

    Note that the struct update syntax uses = like an assignment; this is because it moves the data, 
    just as we saw in the “Variables and Data Interacting with Move” section. In this example, we can no longer use user1 as a whole after creating user2 because 
    the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the 
    active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the 
    Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply. We can still use user1.email in this example, since its value was not moved out.


*/


/*
    You can also define structs that don’t have any fields! 
    These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section. 
    Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
*/


/*

    Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).


*/