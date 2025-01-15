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

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // will give an error as only one mutable reference can exist.
    println!("{}", r1);
    /*
        This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. 
        The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, 
        we tried to create another mutable reference in r2 that borrows the same data as r1.  
    */

    /* In the below example it works because the mutable reference was used before creating another mutable reference */
    let mut ocd = "hello".to_string();
    let r3:&mut String = &mut ocd;
    println!("{r3}");
    let r4:&mut String = &mut ocd; // 


    let mut f = String::from("hello");

    let r5 = &f; // no problem
    let r6 = &f; // no problem1
    let r7 = &mut f; // BIG PROBLEM

    println!("{}, {}, and {}", r5, r6, r7);

    // ALSO NOTE THAT IN ALL CASES THE POPS out only when one of the reference in being used

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");


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