
/*
    Crates are of two types : 
    1) binary OR EXECUTABLE crates so each code we wrote that had a main function is was a executable crate
    2) Library crate, they do not have main function and are intended to be used by other crates

    The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate 


    A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.


    A package can have multiple binary crates and should be places under src/bin each file will be a separate binary crate.

    After we run cargo new my-project, we use ls to see what Cargo creates. In the project directory, there’s a Cargo.toml file, giving us a package. There’s also a src directory that contains main.rs. Open Cargo.toml in your text editor, and note there’s no mention of src/main.rs. Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.



    */

//Declaring modules:
mod garden;
/*
    The compiler will look for the module’s code in these places:
    Inline, within curly brackets that replace the semicolon following mod garden
    In the file src/garden.rs
    In the file src/garden/mod.rs
*/
/*

    Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
    Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    In the file src/garden/vegetables.rs
    In the file src/garden/vegetables/mod.rs


*/

/*
    structure : 
    in the root crate we use module 


*/
fn main(){

}