use std::io;
fn main(){
    let arrd:[&str; 4] = ["ab","bc","cd","de"];
    // let arrd = ["ab","bc","cd","de"];

    println!("{:?}",arrd);

    let mut c = [1,2,3];

    c[2]= 10;
    println!("{:?}",c);
    println!("{}",c.len());

    // the iter function

    for val in c.iter(){
        println!("{}",val);
    }

    let mut bc =[12,2,3,2,4,5];

    for i in 0..bc.len(){
        if bc[i] == 2 {
            bc[i] = 0;
        }
    }

    for j in 0..bc.len(){
        println!("Index is : {}, Value is: {}",j,bc[j]);
    }


    let mut a = [3; 5];
    println!("{:?}",a);
    a[3] = 10;
    println!("{:?}",a);
    // so essentially if i print the a it will print [3,3,3,3,3]
    /*
        The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
     */

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Error reading the input");

    let index: usize= index.trim().parse().expect("Error not a int value");
}


// The way you declare array is with the []