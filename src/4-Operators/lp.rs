fn main(){
    /*
        while 
        loop
        for => bounded loop
    
     */

    // for loop

    for a in 1..20{ // 20 is exculded [1,20)
        if a%2 == 0{
            continue; // sends the control back to the begging of the loop while moving on to the next iteration
        }
        println!("{}",a);
    }

    // an indefinite loop 

    let mut ad = 0;
    while ad<5{
        println!("{}",ad);
        ad+=1;
    }

    let mut c=0;

    loop {
        c-=1;
        println!("{}",c);
        if c == -10 {
            break;
        }
    }

    let mut count:u32=0;
    loop{
        count+=1;
        if count == 3 {
            println!("Welcome to LA");
        }
        if count==5{
            println!("Time to call it a day");
            break;
        }
    }
}