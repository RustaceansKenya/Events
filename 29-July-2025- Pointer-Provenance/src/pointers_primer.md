# Pointers Primer

### References
If you want to understand Rust pointers in general, you can read these... I don't know if they will help you.  
1. For a soft introduction where you get to build linked lists, read the book "[Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)" by [Aria Desires](https://faultlore.com/blah/) 
2. Occasionally skim the [`core::ptr` module docs](https://doc.rust-lang.org/core/ptr/index.html)
3. Occasionally skim the [`primitive Type pointer` docs](https://doc.rust-lang.org/core/primitive.pointer.html)


## Intro to 'pointers'
Pointers are variables that store memory addresses. I know I have just over-simplified it. Kindly, just let me say things without consequences, we are all devs here. Please, I just want to teach the intuitive part, not the technical part.    

There are many kinds of pointers: smart-pointers, raw-pointers, reference-pointers... others that the author doesn't know yet...  


1. **Raw-pointers** in Rust are denoted with the symbols `*const T` and `*mut T`. These are pointers that are so bare that they literally store an integer.  
2. **Reference-pointers** are denoted by `&mut T` and `& T`. My assumption is that you already know what they are and how to use them.
3. **Smart Pointers** are pointers that don't just store addresses, they have metadata and extra control mechanisms. You can read about smart pointers [here](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).  