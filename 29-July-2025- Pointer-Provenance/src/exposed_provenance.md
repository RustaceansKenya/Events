# Exposed Provenance

(The author has not understood Exposed provenance to the point where they can write about it)  
(someone, kindly crop in)

But anyway, here is my little understanding

Strict provenance keeps track of the provenance-info and integer-information for each pointer independently. In other words, Strict provenance is all about binding each pointer with its own provenance-metadata.  
However, in exposed provenance, that binding mechanism is not strictly enforced.

In exposed provenance, there is a globally shared database that stores **exposed** provenance instances.  
So the programmer can deviate from strict provenance and dump a **provenance instance** into this global database... afterwards, the programmer can fetch a provenance instance from this database and bind it to a new different pointer.  

The problem is that there is no algorithm that decides which provenance will be fetched from the database. You can think of this as “guessing” the right provenance, and the guess will be “maximally in your favor”, in the sense that if there is any way to avoid undefined behavior, then that is the guess that will be taken.  

Anyway, I do not know (forget everything I have just said)  


What?  
It must have been the wind.  
