
// the optional enum with genic type is already available with us in the prelude so we do not need to implement or call it explicitly 

#[derive(Debug)]
enum withmatch{
    one,
    two,
    three
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn printHorrahifQuatorOfAlaska(&self)-> String{
        match self {
            Coin::Quarter(UsState::Alaska)=>{
                println!("its the fkn coin of Alaska...");
                "Horahhhh".to_string()
            },
            _=> "bye".to_uppercase().to_string()
        }
    }
}
// #[derive(Debug)]
// struct abc{
//     name:String,
//     age:u8
// }

// impl abc {
//     fn makenewObjFromName(&self,_age:u8)->Self{
//         return abc{name:self.name,age:_age}; // will throw error because its like we are trying to copy a reference to the name

//         /*
//             blah.name => "heiii";
//             blah.method(5);
//             abc{name:&(blah.name)}==> "heiii"
        
//          */
//     }

//     // for associated functions use  :: 
// }

fn main(){
    let m = Some(4);
    let z:Option<i32> = None; // so when we use None the compiler will not be able to make out what the type of None is so we will need to specify the type explicitly

    let bc = withmatch::one;

    let mc = match bc{
        withmatch::one=> 1,
        withmatch::two=>2,
        _=> 3
    };

    println!("{:?}",bc);  
    dbg!(bc);
    println!("{mc}");

    let cc = Coin::Quarter(UsState::Alabama);
    println!("{:#?}",cc);

    let dd= Coin::Quarter(UsState::Alaska);
    let str = dd.printHorrahifQuatorOfAlaska();

    println!("{}",str);

    // something important to note that the matches are exhaustive which means all possible cases must be used

    // if let 

    let m = if let Coin::Quarter(UsState::Alaska) = dd{
        1
    }else{
        2
    };

    println!("{m}");


}

fn plus_ONE(x:Option<i32>) -> Option<i32>{
    match x{
        Some(i)=>Some(i+1),
        None=> None
    }
}