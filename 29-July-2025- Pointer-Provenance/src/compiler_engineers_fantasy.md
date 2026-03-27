# A compiler-engineer's fantasy


Two goals of a compiler:
1. Produce Correct artifact code
2. Produce efficient code (according to the user's configs eg focus on performance? Focus on memory usage?)

To achieve the above 2 goals well, the engineers need a lang whose syntax and semantics are so information-rich and explicit to the point that :  
1. Compiler error messages become very specific
2. Optimizations become sure-bets instead of may-be's. A May-be optimization is as good as a wrong optimization. Compilers are conservative.  

## Is Rust's Type system Nice?

Type systems are useful not just for the structural safety-guarantees they provide, but also for helping compilers generate
more efficient code by simplifying important program analysis.  

If the above statement is not clear, read [this small section here](./typesystem_and_its_role_in_compilation.md)

Rust type system is currently not enough... hahaha. This is a personal opinion. Please, I did not intend to say such an unspeakable sentence.  
The borrow-checker cannot statically track the **provenance** of unsafe pointers.  
The borrow-checker cannot statically prove the correctness of unsafe code.  


