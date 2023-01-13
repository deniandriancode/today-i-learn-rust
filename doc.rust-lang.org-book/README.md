# Hello, World!  

Rust files always end with the _.rs_ extension. For example, `main.rs` or `hello.rs`, if you are using more than one word in your filename, the convention is **to use underscore to separate them**. For example `hello_world.rs` rather than `helloworld.rs`.  

Rust is a compiled programming language, meaning that you need to compile the source file first before running it. To compile rust file use `rustc` followed by your source file name.

``` sh
rustc main.rs
```  

the command above will produce `main` executable file, to run it type `./main`, now you should see

``` sh
Hello, world!
```  

printed on the terminal.  

Four important details to notice:  
1. Rust style is to indent with four space, not a tab
2. `println!` is called a Rust macro. If it had called a function instead, it would be entered as `println`(without the `!`)
3. We pass the string `"Hello, world!"` as an argument to the `println!`, and the string is printed to the screen
4. We end the line with a semicolon (`;`), which indicate that this expression is over and that the next one is ready to begin.

