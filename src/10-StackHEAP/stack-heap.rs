fn main(){
    let b  = if true {};
    println!("{:?}",b);


    /*
        All data stored on the stack must have a known, fixed size.
        Data with an unknown size at compile time or a size that might change must be stored on the heap instead.    

        The heap is less organized: when you put data on the heap, you request a certain amount of space. 
        The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, 
        and returns a pointer, which is the address of that location. 

        This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating)


        Each value in Rust has an owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
    */


}