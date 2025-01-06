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
}