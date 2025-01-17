#[derive(Debug)]
struct Rectangle{
    height:u32,
    width:u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.height*self.width
    }
    fn can_hold(&self,other: &Rectangle)->bool{
        self.area() > other.area()
    }
}

fn main(){
    let r1 = Rectangle{
        height: 30,
        width : 200
    };

    let r2 = Rectangle{
        height:  10,
        width : 20
    };

    let bb = r1.can_hold(&r2); // owns the value true

    println!("Rectangle r1 can hold Rectangle r2 {bb}");
    
    let cc = &r1.can_hold(&r2); // has the reference to the value true
    
    println!("Rectangle r1 can hold Rectangle r2 {}",cc);

    let mc = Rectangle::init(10, 5);
    dbg!(&mc);
    let area = mc.area();
    println!("{area}");

}


// always remember debug takes the ownership of an expression 

// there can be multiple impl blocks 

/* 
    There can also be functions inside impl block which do not take (self) or (&self) or (&mut self) as an argument such functions are called Associated Functions
*/

// example of both the above comments

impl Rectangle {
    fn init(h:u32,w:u32) -> Self{ // self is actually short for self: &Self
        Self{
            height:h,
            width:w
        }
    }
}

// associated functions are called using the path resolution operator :: 