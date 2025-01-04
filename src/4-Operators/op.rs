fn main(){
   /*
   
        Arithmetic
        bitwise
        Comparison
        Logical && || !
        Conditional
   
    */ 

    /*
    
       if , if else , else if , match statement 
    
    
     */

    let user = "todd";

    if user.len() == 3{
        println!("hiiii");
    } else{
        println!("blahhh");
        
    }


    // Match Statement 

    let micro = "xc12";

    let body_part = match micro {
        "xc12" => {println!("Found  the match"); "Tummy Biome"},
        _=>"unknown"
    };

    println!("{}",body_part);

    let aa = 3;
    let bb=4;
    if aa<bb && aa>6{
        println!("fail");
    } else if aa<bb||aa>6{
        println!("success");
    }else{
        println!("Please Try again");
    }

}