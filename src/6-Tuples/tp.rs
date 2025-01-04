fn main(){
    // tuple example

    let ff:(i8,f32,i32) = (2,2.2,100);
    println!("{:?}",ff);

    println!("{:?}",ff.0);

    let jj:(i32,bool,&str) = (100,true,"Hello");   
    blah(jj);
}

fn blah(tp:(i32,bool,&str)){
    let(a,b,c) = tp;
    println!("{}\n{}\n{}",a,b,c);
}