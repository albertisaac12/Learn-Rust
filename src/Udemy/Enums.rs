

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
}

/*
    Enums allow a variable to have a set of predefined constants assigned to them 
    The variable must be equal to one of the predefined constants
    Enum doesn't implement the Display trait 


*/