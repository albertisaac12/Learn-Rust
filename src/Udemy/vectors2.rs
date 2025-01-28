fn main(){
    
    
    let mut vec1:Vec<u32> = Vec::new();

    vec1.push(1);

    let v:Vec<u32> = vec![1,2,3,4];

    let third = v[2];

    println!("{}",&third);

    let third = v.get(2);

    match third {
        Some(bc) => println!("{}",bc),
        None => panic!()
    }

    // let v = vec![String::from("hello"), String::from("world")];
    // let third = v[1]; // This would cause a move since `String` does not implement `Copy`. // this will also throw an error


    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6); // this will fail because we cannot have a mutable refrerance after an immutable reference

    // println!("The first element is: {first}");


    let v = vec![100,1,5];
    
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![15,5,9,6];

    for i in &mut v {
        *i+=50;
    }
    
    /*
        To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
    */
}