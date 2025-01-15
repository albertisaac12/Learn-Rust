fn main(){

    let vecter1 = vec![2,1,4];
    // vector vector1 owns the object in the heap;
    // only a single variable owns the heap memory at a given time

    let vector2 = vecter1;

    println!("{:?}",vector2);

    let vecter3 = vec![5,6,7];
    dp(vecter3);
    // println!("{:?}",vecter3); // this will give an error becasue the ownership of the vector3 was taken by y in when vector3 was passed as an argument to deal with it pass the reference instead
    let vecter4 = vec![7,8,9];
    dn(&vecter4);
    println!("{:?}",vecter4[0]);

    // excercise

    let mut ac = String::from("Hello, ");
    appendTostr(&mut ac);
    println!("{ac}");

}

fn dp(y:Vec<i32>){
    println!("{:?}",y);
}
fn dn(y:&Vec<i32>){
    println!("{:?}",y); // note rust does the dereferencing here
}

fn appendTostr(x: &mut String){
    x.push_str("bye");
}