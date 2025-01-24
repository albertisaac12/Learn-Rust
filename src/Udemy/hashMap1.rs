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
}