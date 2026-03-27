# Provenance

Pointers are not just integers. That is a wrong abstraction.  
If we treat pointers as integers, we get things wrong.  
We need to define a type called 'Pointer' and another called 'Integer'. They may look and act similar in certain scenarios, but they are not usually the same.  

I have a joke : I will treat pointers like true intergers for no reason at all.  
```rust
fn main(){
    let x :u32 = 10; // create a value
    let x_ptr = &x as *const u32; // get the pointer
    let x_ptr_usize = x_ptr as usize; // get the actual address : eg 0x199901
    let x_ptr_u8 = x_ptr_usize as u8; // 64-bit number can be converted to a u8
    let x_ptr2 = (x_ptr_u8 as usize) as *const u32; // convert back to ptr

    unsafe {
        println!("old address of x = {}", x_ptr_usize);
        println!("new address after conversion between diffefent kinds of integers: {}", (x_ptr_u8 as usize));
        println!("new garbage value of x = {}", *x_ptr2);
    };
}
```  

That was not a joke btw, stuff like this happen in long pointer-arithmetic code, or in code that exploits pointer tagging.  
The above situation has an obvious mistake that can be easily seen, but in real life, it won't be that easy to find deep in pointer-arithmetic jungles.  
Life lesson: Pointer != integer.  


Provenance is the attached metadata to a pointer type.  

A pointer has an integer part, and a provenance part.  

The integer part is literally the address it directly points to.  

In Rust, the provenance part of a pointer is the combined information on : 
1. The set of memory addresses that the pointer is allowed to access in an [allocation](https://doc.rust-lang.org/core/ptr/index.html#allocated-object) (this obviously includes the exact address of that pointer) - this is **spatial** information.
2. The lifetime during which the pointer is allowed to access those memory addresses (referred to as Temporal aspect of provenance)
3. Whether the pointer may only access the memory for reads, or also access it for writes.  


If a pointer has the above information available, you can say that *That pointer has provenance*.  
If a pointer is missing the above information, you can say that that pointer *does not have provenance* - that pointer is as good as just another integer.  



## Why Do we need provenance
This information will help us track pointers, ALL pointers. (smart pointers, references, and raw pointers)  
We will see their lifetimes, their overlapping allocations, their access-rights.... and we will be able to track them...

Finally, we are here!  
Us those!  
<br><br><br>
Now people are happy. Get up from that chair and jog with excitement!  
NOW!🫵 Yes, I am talking to you! 🫵
<video controls src="imgs/Get ready, get ready, ready to Jump ~ Otieno Kajwang.mp4" title="Title"></video> 


