# Semi-Bad-Good news

I have some semi-bad-good news, so please stop jogging & celebrating.  
So ... Rust people are still trying to come up with a clean implementation of provenance.(and they are winning) (Shout-out to [Aria Desires and the team](https://github.com/rust-lang/rust/issues/95228))  


The idea is to :
1. Find a way of annotating provenance for each pointer
2. Find a way of determining when undefined behaviour is introduced in the provenance-annotated code.  

So far, in the Rust ecosystem, we haven't found a way to **statically** implement the above 2 ideas. The current method is **Dynamic** and **non-exhaustive**.  

## Idea 1: Finding a way of annotating provenance for each pointer

In Rust, the compiler provides explicit functions for passing on provenance from one pointer_with_provenance to another pointer_without_provenance(in this case, just a usize_value cosplaying as a pointer).  
The problem is that the internal mechanism of this passage is not fully defined.  

When an allocation is created, that allocation is given a unique Original Pointer. For `alloc` APIs this is literally the pointer the call returns, and for local variables and statics, this is the name of the variable/static. An allocation can have many pointers pointing to it (aliasing), but there will always be that one unique Original pointer in that list of aliasing pointers.  

You can derive new pointers and references through borrowing or casting from the original pointer. You can also derive a new pointer from an already derived pointer. Something like : `Original_pointer <-- first_derivative_pointer <-- new_child_pointer`

Every pointer derived from an allocation inherits provenance from their direct parent. They either get the same provenance, or less-provenance ie Borrowing immutably from an original pointer will make sure the new pointer gets read-access instead of full-read-write access. 

Think of it like a sandbox.  
> Some operations may shrink the permissions of the derived provenance, limiting how much memory it can access or how long it’s valid for (i.e. borrowing a subfield and subslicing can shrink the spatial component of provenance, and all borrowing can shrink the temporal component of provenance). However, no operation can ever grow the permissions of the derived provenance: even if you “know” there is a larger allocation, you can’t derive a pointer with a larger provenance. Similarly, you cannot “recombine” two contiguous provenances back into one (i.e. with a fn merge(&[T], &[T]) -> &[T])

There is no escalation.  


Rust has 2 new APIs to help in distributing provenance between parent_pointers and their child_pointers : [`Strict Provenance API`](https://doc.rust-lang.org/stable/std/ptr/index.html#strict-provenance) and [`Exposed Provenance API`](https://doc.rust-lang.org/stable/std/ptr/index.html#exposed-provenance). We will cover them momentarily.  

## The Bad news Explained

People tried to make the tracking static. But 2 facts remained:
1. **Pointers in the real world are too dynamic.** Raw pointers can come from anywhere (*eg* FFI, memory-mapped I/O), and their behavior may depend on runtime conditions that may be too unique for a static analyzer to analyse. Static analysis is not comprehensive enough to account for these arbitrary runtime situationships.  

2. **Compiler Optimizations are so many (and increasing)** : As discussed in the previous sections, there are so many optimization strategies, and each of these optimizations may wrongly optimize code that has raw-pointers due to the `unsure` behaviour of pointers. You could use indicators like `volatile` to prevent optimizations though...good luck inventing and putting anti-optimization flags in your code. Please take that journey for the rest of us to learn from you.  

So far, the Rust compiler cannot compile with Provenance-checks in a **STATIC** manner. However, the rust people came up with  unsafe-code-guidelines, Lints, [miri](https://github.com/rust-lang/miri?tab=readme-ov-file) and [stacked borrows](https://plv.mpi-sws.org/rustbelt/stacked-borrows/), [tree-borrows](https://perso.crans.org/vanille/treebor/)  


## So what are these APIs?  

Here we go...