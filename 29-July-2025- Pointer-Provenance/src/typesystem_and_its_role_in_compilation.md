# Type system and its role in compilation

A type-system in a programming language is a system that defines **and** enforces:
  - the types (kinds of data layouts) *eg* a `&mut usize`  
  - the operations allowed on the types *eg* you can dereference a `&mut usize`.
  - the conditions and contexts needed to perform those operations on the types : *eg* you cannot create two `&mut usize` types that reference the same data in the same scope.  

Rust has a static type-system... one that enforces it's rules at compile time.  

### Buzzword...
**Structural safety** refers to the ability of a program to avoid undefined behavior, memory corruption, or logical inconsistencies due to improper manipulation of data in the data-types.  
A type system ensures structural safety, by construction. You could say, the type-system **IS** the structure itself. 


### The Rust type system and how it ensures structural safety
This is not comprehensive description of the rust type-system... you can find that in the language reference.  
In fact, I am just going to list things so that you can get the idea that the type-system is made up of many mechanisms and that we can improve it by adding more mechanisms.  

1. **The Ownership mechanism**: defines aliasing rules and eliminates data-races
2. **Lifetime annotation mechanism**: statically takes care of dangling pointers...
3. **Algebraic Data Types (ADTs) and Exhaustiveness Checking**: forces programmer to handle all arms of an enum or match statement thereby reducing logical bugs. 
4. **No Null or Uninitialized Values**: Null drama is avoided. Rust’s `Option<T>` forces explicit handling of absence, eliminating null pointer dereferences (a billion-dollar mistake)

This list could go on forever... i just hope you get it.  

### Code optimization and Efficient code generation
For the compiler to generate efficient code, it has to do some optimizations. Simple.  
Largely, optimization is about re-ordering and eliminating reducing memory operations.  
Optimizations should not change the behaviour of the program...nor should they introduce undefined behaviour. They should only make the code more efficient.  

For the compliler to enforce rigorous optimization techniques, it needs metadata on how the program works... and the type-system provides such info. The more precise info the compiler has, the more optimizations it can confidently implement. (ps, that is why type-dependent languages are sometimes seen as a holy-grail for optimization, quite an interesting off-topic)  

In the name of verbosity, here are some examples of how the rust-type system assists in providing info for optimization: 

1. The ownership mechanism affirms to the compiler that it can **re-order** instructions that contain references in certain contexts *eg* the compiler can freely re-order instructions that contain immutable borrows to some variable `x` in the same scope... this is because it is sure that the value in `x` could not have possibly changed so each memory-read in the same scope can be interchanged.  
   
2. **Dead code elimination**: The type system can prove certain code paths are unreachable (*eg* match on an enum where some variants are statically impossible), allowing the compiler to remove such code entirely. (type-state programming, I wish I knew this stuff)
   
3. **Memory Layout Optimization**: the type system can provide info on types such that the compiler can allocate on the stack instead of the heap when lifetimes are provably short (via escape analysis).  
   
4. **Compile time monomorphization**: generic implementations get implemented at compile-time for each associated type such that dynamic dispatch (indirection) is totally eliminated. This als*eg*o ensures `type-specific` optimizations ...*eg*: `Vec<i32>` and `Vec<f64>` generate separate code, allowing for type-specific optimizations (*eg* CPU vectorization for numeric types)
