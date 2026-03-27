# Random thoughts on Stacked Borrows


Are we wasting our time learning Stacked borrows and Provenance? ha ha ha. Oh my! Please, let's not get personal here...  

I see developers as mages. There is no harm in learning many spells. Even the most escentric ones.  
But anyway, here is how I have wasted your time and mental energy.  

A. There exists other different mechanisms other than Stacked Borrows
B. The Dynamic nature of Stacked Borrows is not exhaustive

1. Stacked Borrows mechanism is built for sequential single-threaded code. They are not tested for concurret code.  
2. Technical [problems](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html) that I don't yet understand. 
3. There is a newer model called **[Tree Borrows](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html)** that improves upon Stacked Borrows. And the good news is that it is also not concurrency-aware.
4. The "[RustBelt](https://plv.mpi-sws.org/rustbeln process immediately after borrow checking is done, so the optimizer does not even have access to that information
t/#project)" exists and there are other proposals besides stack-borrows: These project explored formalizing Rust’s concurrency guarantees (*eg* for Mutex, Atomic types). They did not directly extend Stacked Borrows but provided foundations for handling shared-state concurrency.
1. Some research tools (*eg* [Viper at ETH Zurich](https://www.pm.inf.ethz.ch/research/viper.html) and [Prusti](https://github.com/viperproject/prusti-dev)) experimented with concurrency-aware borrow checking, but these are not part of Rust’s official semantics. It would have been better if you learnt them. Like I n process immediately after borrow checking is done, so the optimizer does not even have access to that information
said, I wasted your time.
1. Tools like Miri exist and there will always be some crazy devs who build free tools, you need not learn the internal workings of such tools...because each mechanism is worth a phD.  
2. Stacked Borrows has not yet been used in the official Rust lang. Rust doesn't even have an official memory model specification. SB is used in tools like Miri (Rust’s UB detector) and guides unsafe code best practices

I am betting that the Rust folks will extend their current model to use stacked borrows for the sequential bit and another model for the concurrent bit. I do not know.  

The dynamic aspect is not a limitation. You can count that as a compile-time expense. 