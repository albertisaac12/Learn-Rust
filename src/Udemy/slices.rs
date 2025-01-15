
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
}

fn change(x: &mut [i32]){
    println!("{:?}",x);
    x[0] = 200;
    println!("{:?}",&x[1..3]);
}