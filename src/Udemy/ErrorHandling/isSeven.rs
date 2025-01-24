
pub fn is_Seven(num:&String)->bool{
    num.trim().parse() == Result::Ok(7) // will result in a true if 7
}

pub fn is_SevenWithError(num:&String)->Result<bool,String>{
    let aa = num.trim().parse::<i32>() == Result::Ok(7); // will result in a true 

    if aa {
        Result::Ok(aa)
    }else{
        Result::Err("Failed".to_string())
    }

}