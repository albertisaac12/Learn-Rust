use std::collections::HashMap;

fn main(){
    let mut accountInfo =  HashMap::new();
    accountInfo.insert("blah1", 15);
    accountInfo.insert("blah2", 16);
    accountInfo.insert("blah3", 17);
    
    println!("{}",accountInfo.len());
    // println!("{}",accountInfo.capacity());
    
    accountInfo.remove(&"blah3");
    
    println!("{}",accountInfo.len());

    println!("{:?}",accountInfo["blah1"]);

    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}