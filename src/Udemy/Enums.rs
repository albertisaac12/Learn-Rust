

#[derive(Debug)]
enum days {
    monday,tuesday,wednesday,thursday,friday,saturday,sunday
}

#[derive(Debug)]
enum withTypes{
    v1(String),
    v2(u32),
    v3(days),
    V4(u8, u8, u8, u8),
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// struct city {
//     name:String,
//     day:days
// }

// impl city {
//     fn gen(name:String,day:u32)->Self{
//         Self{
//             name:name,
//         }
//     }
// }
fn main(){
  let _day = days::friday;

  let _v1 = withTypes::v1(String::from("hello"));
  let _v2 = withTypes::v2(10);
  let _v3 = withTypes::v3(days::friday);
  let _v4 = withTypes::V4(1, 2, 3, 4);
  println!("{:?}",_v4);
  
  println!("{:?}",_v3);

//   let x: Option<i32> = Some(5);
// // println!("{}", x + 1); // Error: can't use Option<i32> as i32

// if let Some(value) = x {
//     println!("{}", value + 1); // Works because we handle the Option
// }

// enums with control flow (match)


let _penny = Coin::Penny;
let cents_penny = val_in_cents(_penny);
println!("{}",cents_penny);

let m = 3;

let f = match m {
    1=> 3,
    _=>4
};

println!("{f}");


}

/*
    Enums allow a variable to have a set of predefined constants assigned to them 
    The variable must be equal to one of the predefined constants
    Enum doesn't implement the Display trait 


*/

fn val_in_cents(coin:Coin)->u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}