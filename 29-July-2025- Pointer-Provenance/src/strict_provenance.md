# Strict Provenance

This API provides the following solutions:  
1. It gives the dev **explicit** functions to cast between pointers and integers. You no longer have to do manual casting. This is important because these instructions explicitly indicate whether the casting preserves provenance or not. Tools like Miri can then recompile such code with ease and track when provenance was lost or regained.  
2. It provides a conceptual way of tracking and storing provenance of the pointers like a namespace (this has not been implemented in rustc but in tools like Miri)

Here is the API:  

1. [`ptr::addr()`](https://doc.rust-lang.org/core/primitive.pointer.html#method.addr) : this is an explicit way of casting a pointer into a `usize`. You could do the usual manual cast as shown below...but that's just being dramatic, don't you think so?  
```rust
fn main(){
    let x: u32 = 10;
    let x_ptr: *const u32 = &x as *const u32;  
    let x_usize_addr = x_ptr.addr(); // explicit way of converting pointer to usize
 
    // instead of using manual cast
    let addr_after_manual_cast = x_ptr as usize;
    
    // using manual casts is not as explicit as using .addr(). There is no effect, it's just a matter of principle
    assert_eq!(addr_after_manual_cast, x_usize_addr);
}
```

2. [`ptr::with_addr(self, x: usize) -> ptr`](https://doc.rust-lang.org/core/primitive.pointer.html#method.with_addr) creates a new pointer with the given `usize` address AND copies the provenance of the original pointer to become the provenance of the newly created pointer.  
```rust
fn main(){
 let mut x: [u32; 5] = [10, 20, 30, 40, 50];

 // let's get the address of the array's base, we will use this for some time
 let base_address : usize = (&mut x[0] as *mut u32).addr();

 // Provenance of References is created automatically and is always correct since references cannot be created from invalid memory...
 // However, aliasing references and raw pointers can eventually make a reference invalid.
//  Let's look at references and raw-pointers that demonstrate strict-provenance sand-boxing nature..   

  // pointer_1 (a simple mut reference)
  // integer details --> This pointer points at the 0'th element address, ie. the base of the allocation
  // Provenance details
      //  set of affected addresses: [base, base+1, base+2, base+3, base+4]
      //  validity lifetime : It lives as long as lifetime '1
      //  access rights : read-write each address
  let pointer_1 = &mut x[..]; 

  // pointer_2 (a raw pointer derived from a mut reference)
  // integer details --> This pointer points at the 0'th element address, ie. the base of the allocation
  // Provenance details
      //  set of affected addresses: [base, base+1, base+2, base+3, base+4]
      //  validity lifetime : It lives as long as lifetime of the parent reference ie a lifetime of '1 just like pointer_1
      //  access rights : read-write each address
  let pointer_2 = pointer_1 as *mut [u32]; 

  // pointer_3 : (a raw pointer NOT built from a reference. It is built from another raw pointer)   
  // integer details --> This pointer points at the 0'th element address, ie. the base of the allocation
  // Provenance details
      //  set of affected addresses: [base, base+1, base+2, base+3, base+4]
      //  validity lifetime : It lives as long as lifetime of the parent-pointer ie a lifetime of '1 just like pointer_2
      //  access rights : read-write each address
      let base_address = core::ptr::addr_of_mut!(x).addr(); // no reference involved in address instruction
      let pointer_3 = pointer_2.with_addr(base_address);

  // pointer_4 : (a raw pointer NOT built from a reference. It is built from another raw pointer)
  // integer details --> This pointer points at the 0'th element address, ie. the base of the allocation
  // Provenance details
      //  set of affected addresses: [base, base+1, base+2, base+3, base+4]
      //  validity lifetime : It can live as long as lifetime of the parent reference ie a lifetime of less_or_equal '1 just like pointer_2
      //  access rights : read-write each address
      let base_address = core::ptr::addr_of_mut!(x).addr(); // no reference involved in address instruction
      let pointer_4 = pointer_2.with_addr(base_address);

  // pointer_5 : (a raw pointer NOT built from a reference. It is built from another raw pointer BUT 
  // ...the catch is that it has changed its integer details and maintained provenance details)
  // integer details --> This pointer points at the 2'nd element address, ie. the base+1
  // Provenance details
      //  set of affected addresses: [base, base+1, base+2, base+3, base+4]
      //  validity lifetime : It can live as long as lifetime of the parent reference ie a lifetime of less_or_equal '1 just like pointer_2
      //  access rights : read-write each address
      let base_address = core::ptr::addr_of_mut!(x).addr(); // no reference involved in address instruction
      let address_of_second_element = core::ptr::addr_of_mut!(x[1]).addr(); // we could have done pointer arithmetic but I like this one
      let pointer_5 = pointer_2.with_addr(address_of_second_element);

  
  // pointer_6 : (a raw pointer built from a reference BUT that reference is a mut slice)
  // So the spatial info in the provenance details changes
  // the integer details also change
  // integer details --> This pointer points at the 3rd element address, ie. the base+2
  // Provenance details
      //  set of affected addresses: [ base+2, base+3, base+4]
      //  validity lifetime : It can live as long as lifetime of the parent reference ie a lifetime of less_or_equal '2 just like the parent_slice
      //  access rights : read-write each address
      let parent_slice = &mut x[2..5]; // the slice [30, 40, 50]. LET US ASSUME THIS SLICE HAS '2 LIFETIME
      let pointer_6 = parent_slice as *mut [u32];

      unsafe { assert_eq!(30, (*pointer_6)[0]); }

  // pointer_7 : (a raw pointer NOT built from a reference BUT from a parent-raw-pointer. The catch is ...
  // ... that the Parent-raw-pointer was derived from a slice. Meaning it had a shrinked provenance beforehand, and it ...
  // ... will pass on that shrinked provenance)
  // So the spatial info in the provenance details shrunk
  // the integer details remain the same
  // integer details --> This pointer points at the 3rd element address, ie. the base+2
  // Provenance details
      //  set of affected addresses: [ base+2, base+3, base+4]
      //  validity lifetime : It can live as long as lifetime of the parent_pointer ie a lifetime of less_or_equal '2 just like the parent_pointer borrowed from parent_slice
      //  access rights : read-write each address
      let address_of_third_element = (&mut x[2] as *mut u32).addr();
      let pointer_7 = pointer_6.with_addr(address_of_third_element);

  
  // pointer_8 : (a raw pointer NOT built from a reference BUT from a parent-raw-pointer. There are 2 catches...
  // ...1. that the Parent-raw-pointer was derived from a slice. Meaning it had a shrinked spatial provenance beforehand
  // ...2. We are shrinking the Mutability info in the Provenance details. The new pointer is a read-only pointer
  // So the spatial info in the provenance is unaffected
  // It's the Access Rights that change (Mutability Info)
  // the integer details remain the same
  // integer details --> This pointer points at the 3rd element address, ie. the base+2
  // Provenance details
      //  set of affected addresses: [ base+2, base+3, base+4]
      //  validity lifetime : It can live as long as lifetime of the parent_pointer ie a lifetime of less_or_equal '2 just like the parent_pointer borrowed from parent_slice
      //  access rights : read-ONLY access to each address
      let address_of_third_element = (&mut x[2] as *mut u32).addr();
      let pointer_8: *const [u32] = pointer_7.with_addr(address_of_third_element); // Shrinking Mutability

      // the attempted Mutability escalation below will fail
      // let pointer_9: *mut [u32] = pointer_8.with_addr(address_of_third_element); // compilation error
}
```
3. `ptr::map_addr()` : a sugar coat for `ptr::with_addr()` but with a closure as input. No need to cover it.  


## Problems with Strict Provenance

From the examples above, you can see that we began by extracting provenance information from references. References have provenance. And the problem is that currently, that is the only way to *create* provenance. You cannot define your own Provenance data and pass it on to an integer in order to create a complete pointer.  
You cannot do something like: 
```rust
// We as the programers know that there is a static memory unit that...
// ... contains the value 10 and it is 4-bytes long  
// ... the address of this memory unit is 0x100
// let static x : u32 = 10;

// we could then write something like this...
let provenance_instance_for_x : Provenance<u32>= {
   allocation_base_address : 0x100, // address_of_x,
   allocation_size : 4_bytes, // 32 bits
   lifetime_info: static,
   spatial_info: [0..4], // affect_all_4_bytes of the allocation
   mutability_info: read_only
};

let raw_address = 0x100;

// my_raw_pointer_with_provenance bbuilt without the need for building a reference first
let my_raw_pointer_with_provenance = create_pointer (raw_address, provenance_instance_for_x);
```

Why is it important to be able to define Provenance without the help of references?  
Because there are times when you cannot create a reference, eg when dealing with MMIO  




