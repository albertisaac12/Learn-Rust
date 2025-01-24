fn main(){
    let mut x = 10;

    x = if x>11{
        5
    }else{ 1 };

    println!("{x}");

    let f = false;

    // let m = if f { "six" } else { 9 }; // this will throw an error because of if and else the return types are incompatible when assigning the value
    let mut count =0; 

    let result = loop {
        count+=1;
        if count == 10 {
            break count*2;
        }
    };

    println!("{}", result);

    bb();

    for hh in (1..11).rev(){
        println!("{hh}");
        if hh==1 {
            println!("LIFTOFF");
        }
    }
}

fn aa(x:i32)-> i32{
    let mut y = 0;
    y=x+10;
    return  y;
}


fn bb(){
    let mut count = 0;
    'doing_something : loop{

        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'doing_something;
            }
            remaining -= 1;
        }

        count+=1;

    }
    println!("End count = {count}");

    let mut sd:&str = "sdadada";
    sd ="fuck";

    // let c = if 1 == 1 {2}else {"fuck"};
}