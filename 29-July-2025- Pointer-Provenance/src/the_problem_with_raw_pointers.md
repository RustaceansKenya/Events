# The Problem with raw-pointers  
The key point is that in Rust, references (`&T` and `&mut T`) are guaranteed to never be null and to always point to valid, aligned, and initialized data of the correct type (for the lifetime of the reference).  


The underlying superpower of raw-pointers **over** reference-pointers is that they can point to **ANY** memory address. You are free to access any memory address within your program's address space. That is their super-power.  

This consequently means that the pointer gains 3 caveats that references don't have to put up:  

1. raw-pointers can point to memory that is out-of-bounds (unlike reference-pointers).  
2. raw-pointers can point to unaligned addresses (unlike reference pointers)
3. raw-pointers can point to null

Here are some notes on [memory alignment](./alignment/alignment.md) if you need them.  

Anyway... here are demos of the 3 disasters above

1. Out of bounds access just made us read garbage: 
```rust
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
1. Unaligned memory access just caused a runtime panic, Raw pointers ignore hardware alignment requirements.
```rust
// This code compiles, but it gets a runtime error in x86 machines
fn main() {
    let data: u32 = 0xDEAD_BEEF;
    let addr = &data as *const u32 as usize;
    let unaligned_addr = addr + 1; // Misaligned address (not 4-byte aligned)
    let unaligned_ptr = unaligned_addr as *const u32;

    unsafe {
        // Hardware trap on ARM/x86! Undefined Behavior!
        println!("Unaligned read: {:X}", *unaligned_ptr);
    }
}
```
1. Dereferencing a null pointer is just the perfect recipe for undefined behaviour. I remember being taught this line even before I knew what dereferencing was...I saw this statement everywhere... I only understood it last year (Imagine!)
```rust
fn function_that_accidentally_returns_null_ptr (num: &u32) ->  &u32{
    if *num != 10 {
        return &num;
    }else{
        let null_ptr = core::ptr::null();
        let null_ref = unsafe {&(*null_ptr)};

        return null_ref;
    }
}  

fn main() {
    unsafe {
        // doesn't crash
        println!("Dereferencing possible null: {}", function_that_accidentally_returns_null_ptr(&12));

        // Instant crash or unpredictable behavior
        println!("here comes the bride.... King Kunta Kunte Himuselufu");
        println!("Dereferencing possible null: {}", function_that_accidentally_returns_null_ptr(&10));
    }
}
```
