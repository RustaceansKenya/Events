# Miri

From the past section, it was made clear that `rustc` does not have a method of inspecting whether raw-pointers violate Rust's Borrow-checker rules. 
And that it is [hard to statically detect undefined behaviour](./bad_news.md#the-bad-news-explained)  


## Enter Miri
If you can't yet solve something statically, then you can try doing it dynamically, using an interpreter. Right?  


[Miri](https://github.com/rust-lang/miri?tab=readme-ov-file) did just that.    

Miri is a Rust **interpreter** that dynamically checks for undefined behaviour and [other things](https://github.com/rust-lang/miri?tab=readme-ov-file) in our Rust code.  

Miri employs 👇these two independent methods of checking for aliasing-rules violations:  
1. [Stack Borrows](https://plv.mpi-sws.org/rustbelt/stacked-borrows/)
2. [Tree Borrows](https://perso.crans.org/vanille/treebor/)

We will not discuss these mechanisms here, please read up on them on your own. What we will cover is how to use Miri in your workflow.  
Miri has the ability to store, track and inspect the provenance of both references and raw-pointers.  

The problem with testing something based on dynamic info is that the program may not exhaust all possible excution-paths. Such tests are not exhaustive.  
For this reason, you have to test a program dynamically while subjecting it to **many** test cases/data, hoping to stimulate all paths that the program-execution could have taken.  

## Getting Miri setup
To install Miri, run this command : 
```shell
rustup component add miri
```


## Running Code with Miri
To run the code using Miri as the interpreter, run the following command in your cargo-built repo
```shell
# Instead of running 
# cargo run # this will use rustc instead of Miri

# use this command
cargo miri run
```

## Testing your code 
As earlier noted, subject your code to exhaustive tests because Miri is a dynamic checker. Your Miri-checks will be as good as your tests.  
```shell
# use the command
cargo miri test
```  

## Example
Try running this out-of-bounds example using miri vs rustc.  
You will notice that :
1. `rustc` does not flag any error, it just prints out garbage
2. `miri` catches the error and explicitly informs you that undefined behaviour was brought-about by an out-of-bounds access.
```Rust
fn main() {
    let arr = [1, 2, 3];
    let out_of_bounds_ptr = arr.as_ptr().wrapping_add(15); // Points past array end

    // println!("{}", arr[15]); // Oh my! Compile-time error: index out of bounds

    unsafe {
        // Attempt to read invalid memory - Undefined Behavior!
        println!("Dereferencing ghost: {}", *out_of_bounds_ptr); 
    }
}
```  


