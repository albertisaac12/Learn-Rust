

/*

    Modules allow us to heicharcaly split the code into modules, and manage the visibility (public/private) between them.
    A module is a collection of items.
    functions, structs , traits, impl blocks , and even other modules

    Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:
    Packages: A Cargo feature that lets you build, test, and share crates
    Crates: A tree of modules that produces a library or executable
    Modules and use: Let you control the organization, scope, and privacy of paths
    Paths: A way of naming an item, such as a struct, function, or module


    Multiple modules are compiled into a uint called crate. The cargo tool is used to manage crates in Rust

    use keyword helps us to import a public module.


*/

// note functions are by default private



pub mod songs{
    pub fn play(name:String) {
        println!("{name}, Playing.....");
    }
}

pub mod tracks {
    pub mod rock {
        pub mod indie {
            pub fn select(str:String){
                println!("{str}");
            }
        }
    }
}

use songs::play;
use tracks::rock::indie::select;
fn main(){

    play("Call out my Name".to_string());
    select("Starboy".to_uppercase().to_string());
}
