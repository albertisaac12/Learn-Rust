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
}