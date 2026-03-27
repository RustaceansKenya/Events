# References

Reference pointers stick to Rust's borrow-checker rules which can be oversimplified to: You can have multiple non-mutable references to data at the same time BUT only one mutable reference to the data.... bla bla. You get it.  


They eliminate all those things developers from the past used to experience : Data-races, User-after-free...just things that we now take for granted.  


## **But are references enough?**  

Turns out pure references are not enough for every-day tasks.  
So we have things like 👇these👇 coming into the picture:
1. Interior Mutability
2. Raw Pointers

## **Why would I use such things? Yuck! I am a safe programmer - I spit at unsafety**  
You could argue that everything in the list below could be built by better designs using pure references. The author is a bad designer. Please just tolerate their immaturity without question. Pretty please, don't attack my ego.

1. **Shared Ownership** There are data structures that contain objects that are truly owned by multiple parents eg A fully connected graph. (utilizes interior mutability)
2. **Concurrent Shared State**: For thread-safe shared mutation, `Arc<T>` (Atomic Reference Counting) + `Mutex<T>` combine shared ownership with synchronized access. (utilizes interior mutability)
3. **Foreign Function Interface (FFI)**: Aaaah, you are forced to use some foreign library together with your perfect Rust code.
```rust
extern "C" {  
    fn c_process_buffer(ptr: *mut u8, len: usize);  
}  

fn main() {  
    let mut data = vec![0u8; 1024];  
    unsafe {  
        // Convert Rust Vec to raw pointer + length  
        c_process_buffer(data.as_mut_ptr(), data.len());  
        // Reclaim ownership after FFI call  
        let _ = Box::from_raw(data.as_mut_ptr());  
    }  
}  
```


4. **Implementing memory-mapped I/O or hardware registers** : You have addresses, now you have to turn them into pointers and references. Very sad. 
```rust
const UART_TX: *mut u8 = 0x1000_0000 as *mut u8;  

fn write_serial_byte(byte: u8) {  
    unsafe {  
        // Direct memory-mapped I/O  
        ptr::write_volatile(UART_TX, byte);  
    }  
}  
```


5. **Self-Referential Data Structures** : Building intrusive lists or graph nodes with backlinks. Parent-child cycles violate borrow checker's rules 
```Rust
struct Node {  
    value: i32,  
    next: *mut Node,  // Raw pointer to avoid ownership  
    prev: *mut Node,  
}  

fn link_nodes(a: &mut Node, b: &mut Node) {  
    a.next = b as *mut Node;  
    b.prev = a as *mut Node;  
}
```  


## Okay, turns out I can't truly escape raw-pointers... but
"But I can spend as little time as possible in unsafe section, wrap it up safely and go back to my perfect world!" - me after reading the Rust book.  


Yes, you could do that. Everything is simple and straight-foward until reality happens. We are developers. We are developers. Repeat after me: You are a developer. You are a good developer. You know what I mean. Clean code is a disgusting myth.  

I hear there is a language called ATS, and ADA and Idris2 with much better restrictions that enforce even safer code. ATS even comes with verification. Grass is so green there... aaah...Maybe one day... one day we will get there.  