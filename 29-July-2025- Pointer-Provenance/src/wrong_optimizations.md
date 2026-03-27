# Wrong Optimizations


### Example 1: Constant Propagation Optimization and re-ordering optimization go wrong
```rust

//  ALl the Optimizations below are wrong... but 
// ... but they could have been right if borrow-checker rules were followed


fn example1 (x: & mut i32 , y: & mut i32 ) -> i32 {
    * x = 42;
    *y = 13;
    return *x; 
}

// FACTS from the above code:
// 1. We expect this function to always return 42
// 2. The code involving y should have no effect on x... and vice versa


// Optimization 1 : Constant propagation
// Using constant propagation, `example1` can be reduced to...
fn example1 (x: & mut i32 , y: & mut i32 ) -> i32 {
    * x = 42;
    *y = 13;
    return 42; // this looks like a small optimization
               // but it saved us an entire memory-read
}

// Optimization 2 : Re-ordering instructions
// We have re-oredered instructions without changing behaviour...
// It is a good cache-aware practice to cluster instructions that...
// ...affect the same variable together.
fn example1 (x: & mut i32 , y: & mut i32 ) -> i32 {
    *y = 13;
    * x = 42; // we have re-ordered instructions in order..
    return *x; // ...to cluster instructions that affect overlapping memory regions
            //    good cache-aware practice
}


// COMBINING OPTIMIZATIONS
// The compiler usually applies many optimizations on the same ...
// ...piece of code.  
// Here is our final piece of code after applying the above 2 optimizations
fn example1 (x: & mut i32 , y: & mut i32 ) -> i32 {
    *y = 13;
    * x = 42;
    return 42; 
}

// We are able to make the above 2 optimizations because...
// ...we assumed that x and y were pointing to 2 different memories
// What if they were pointing to the same memory?  

// You are thinking that is impossible?  
// You are laughing and I am serious?
// what if I wrote the main function like this?👇
fn main () {
    let mut local = 5;
    let raw_pointer = & mut local as * mut i32 ;
    let result = unsafe { example1 (& mut * raw_pointer , & mut * raw_pointer ) };
    println !( " {} " , result ); // without the 2 optimizations, this prints "13"!
    // if we apply the 2 optimizations we get 42, which is the wrong answer.
}

```


There are dozens of optimization mechanisms in a compiler... and new clever techniques get discovered from time to time.  
We do not know which optimizations affect which unsafe patterns in which way.  

I will focus on this page's sample from here on...  
I ain't covering the entire LLVM optimization techniques.  