# Hello, World!  

Rust files always end with the _.rs_ extension. For example, `main.rs` or `hello.rs`, 
if you are using more than one word in your filename, the convention is **to use underscore to separate them**. 
For example `hello_world.rs` rather than `helloworld.rs`.  

Rust is a compiled programming language, meaning that you need to compile the source 
file first before running it. To compile Rust use `rustc` followed by your rust file name.

``` sh
rustc main.rs
```  

the command above will produce `main` executable file, to run it type `./main`, you 
should see `Hello, World!` printed on the terminal.
