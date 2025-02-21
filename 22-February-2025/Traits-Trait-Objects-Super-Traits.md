## Traits, Trait Objects and Super Traits

A trait in Rust defines a set of ***behaviors*** or ***capabilities*** that a type can implement. 

By implementing a trait, a type ***shares these behaviors with other types*** that also implement the same trait, enabling code reuse and polymorphism (**many forms**).  

### 1. **Syntax**

- Simple Trait

  ```rust
  trait TraitName {
      // Required methods (no implementation provided)
      // All methods are public so no need to add `pub` in the beginning
      fn required_method(&self);
  }
  ```

- Trait with associated type

  Associated types allow traits to define placeholders for types that are specified when the trait is implemented. This enables traits to work with generic types while still enforcing type constraints.

  ```rust
  trait TraitName {
      // Associated types (optional)
      type AssociatedType;
  }
  ```

  

- Trait with default method
  ```rust
  trait TraitName {
      // Associated types (optional)
      type AssociatedType;
  
      // Default methods (with implementation provided)
      fn default_method(&self) {
          // Default implementation
      }
  }
  ```

  Example is the `TryFrom` trait

  - Accessing methods
    Trait methods are accessed using `.` method

    ```rust
    trait Foo{
    	fn bar();
    }
    
    <Rust Type>.bar() // Accessing method `bar()` on a rust type that implements `Foo`
    ```

    

2. Simple examples

   
   ```rust
   fn main() {
       let foo = Bar {
           name: "RustaceansKenya".to_string(),
       };
   
       println!("{}", foo.get_name());
   
       // Accessing methods that don't take &self as parameter
       println!("Category: {}", <Bar as Foo>::citizenship());
   
       // `id` method is always accesible since it has a default
       // implementation
       println!("ID: {}", foo.id());
   }
   
   trait Foo {
       fn get_name(&self) -> String;
   
       fn citizenship() -> String;
   
       fn id(&self) -> String {
           self.get_name().clone() + " is identified as human."
       }
   }
   
   struct Bar {
       name: String,
   }
   
   impl Foo for Bar {
       fn get_name(&self) -> String {
           self.name.clone() + " is a citizen."
       }
   
       fn citizenship() -> String {
           "Citizen of earth".to_string()
       }
   }
   ```

   ```sh
   #Console Output
   RustaceansKenya is a citizen.
   Category: Citizen of earth
   ID: RustaceansKenya is a citizen. is identified as human.
   ```

   ### `default` trait methods can be overridden by implementing a custom method for a type

   ```rust
   
   trait Foo {
       fn get_name(&self) -> String;
   
       fn citizenship() -> String;
   
       fn id(&self) -> String {
           self.get_name().clone() + " is identified as human."
       }
   }
   
   struct Bar {
       name: String,
   }
   
   impl Foo for Bar {
       fn get_name(&self) -> String {
           self.name.clone() + " is a citizen."
       }
   
       fn citizenship() -> String {
           "Citizen of earth".to_string()
       }
   
       // Here we override the default implementation
       fn id(&self) -> String {
           self.get_name().clone() + " is identified an alien from earth."
       }
   }
   ```

   

   - Traits with an associated type
     ```rust
     fn main() {
         let foo = Bar {
             name: "RustaceansKenya".to_string(),
         };
     
         // `id` method is always accesible since it has a default
         // implementation
         println!("ID: {}", foo.id());
     }
     
     trait Foo {
         type Item;
     
         // Force a trait to always return a certain type
         // as defined by `type Item;` above
         fn id(&self) -> Self::Item;
     }
     
     struct Bar {
         name: String,
     }
     
     impl Foo for Bar {
         // Make the return type a String
         type Item = String;
     
         fn id(&self) -> String {
             self.name.clone() + " is identified as human."
         }
     }
     ```

   - Super traits example
     ```rust
     fn main() {
         let foo = Bar {
             name: "RustaceansKenya".to_string(),
         };
     
         // `id` method is always accesible since it has a default
         // implementation
         println!("ID: {}", foo.id());
         println!(
             "Inception: {}-{}-{}",
             foo.timeline().2,
             foo.timeline().1,
             foo.timeline().0,
         );
     }
     
     impl Explorer for Bar {}
     
     // Create a super trait by using the Add `(+)` sign
     // on two traits
     trait Explorer: Baz + Foo {}
     
     trait Baz {
         fn timeline(&self) -> (u8, u8, u16);
     }
     
     trait Foo {
         fn id(&self) -> String;
     }
     
     struct Bar {
         name: String,
     }
     
     impl Foo for Bar {
         fn id(&self) -> String {
             self.name.clone() + " is identified as human."
         }
     }
     
     impl Baz for Bar {
         fn timeline(&self) -> (u8, u8, u16) {
             (14, 10, 2023)
         }
     }
     ```

     

3. Trait Objects

   > Excerpt from Rust Book:
   >
   > [Polymorphism](file:///home/user0/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch18-01-what-is-oo.html#polymorphism)
   >
   > To many people, polymorphism is synonymous with inheritance. But it’s actually a more general concept that refers to code that can work with data of multiple types. For inheritance, those types are generally subclasses.
   >
   > Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called *bounded parametric polymorphism*.

   ### Example

   ```rust
   fn main() {
       let mut list_store = Communities {
           list: Vec::default(),
       };
   
       list_store.list.push(CodeCommunity::new("RustacensKenya"));
       list_store.list.push(CodeCommunity::new("Go Kenya"));
       list_store.list.push(CodeCommunity::new("Flutter Kenya"));
   
       list_store.print_all();
   }
   
   trait Foo {
       fn id(&self) -> String;
   }
   
   pub struct Communities {
       pub list: Vec<Box<dyn Foo>>,
   }
   
   impl Communities {
       fn print_all(&self) {
           for community in &self.list {
               println!("{}\n", community.id())
           }
       }
   }
   
   pub struct CodeCommunity(String);
   
   impl CodeCommunity {
       fn new(name: &str) -> Box<Self> {
           Box::new(Self(name.to_string()))
       }
   }
   
   impl Foo for CodeCommunity {
       fn id(&self) -> String {
           self.0.clone() + " is a community of coders from Earth"
       }
   }
   ```

   

4. Derive Traits using proc macros
   In Rust it is common to use `#[derive()]` proc macros to auto-derive traits for common Rust types. For example we can implement default types for a struct and enum using the `std::default::Default` trait from the Rust standard library.

   ```rust
   pub struct Communities {
       list: Vec<String>,
       status: Option<String>,
       numbers: u16,
   }
   
   impl Default for Communities {
       fn default() -> Self {
           Self {
               list: Vec::default(),
               status: Option::default(),
               numbers: u16::default(),
           }
       }
   }
   
   enum Planet {
       Earth,
       Mars
   }
   
   impl Default for Planet {
       fn default() -> Self {
           Planet::Earth
       }
   }
   ```

   With such types that implement the defaults from the standard library, we can derive their default types using derive attribute `#[derive(Default)]` instead.
   ```rust
   #[derive(Default)]
   pub struct Communities {
       list: Vec<String>,
       status: Option<String>,
       numbers: u16,
   }
   
   #[derive(Default)]
   enum Planet {
       // Enums need default variant to ne defined by this annotation
       #[default]
       Earth,
       Mars,
   }
   ```

   

5. Auto traits
   The Rust standard library automatically implements some traits for most types especially primitive types like numbers, borrowed strings, Strings, etc. Some of these are Marker traits that can be found at [https://doc.rust-lang.org/std/marker/index.html](https://doc.rust-lang.org/std/marker/index.html).

   For example, the integers automatically implement `Copy` trait meaning that they can be copied around cheaply since they are stored on the stack. Another example is a `String` which implements `Clone` trait only since it is stored on the heap and is expensive to duplicate. Integers also implement the `Add` trait from [std::ops](https://doc.rust-lang.org/std/ops/index.html) allowing addition operations to happen automatically for example:
   ```rust
   1u32 +1
   2000u32 + 999
   ```

   

6. Implementing traits for custom types
   As seen above most Rust standard library types auto-implement certain traits like https://doc.rust-lang.org/std/ops/index.html that allow mathematical traits by overloading operators. Rust enables us to implement such trait for user defined types. For example, how do we add two structs? Just as we would `1u8 + 1`

   ```rust
   // Bring the trait to scope
   use std::ops::Add;
   
   fn main() {
       let community1 = Community::new("RustKe", 500);
       let community2 = Community::new("Flutter", 1500);
   
       let sum = community1 + community2;
   
       println!("Sum of all community members is: {}", sum);
   }
   
   pub struct Communities(Vec<Community>);
   
   pub struct Community {
       name: String,
       // The number of registered members
       numbers: u16,
   }
   
   impl Community {
       fn new(name: &str, numbers: u16) -> Self {
           Self {
               name: name.to_string(),
               numbers,
           }
       }
   }
   
   impl Add for Community {
       type Output = u16;
   
       fn add(self, rhs: Self) -> Self::Output {
           // Since we cannot add 2 names together
           // we will add the `numbers`
           self.numbers + rhs.numbers
       }
   }
   ```

   

7. Iterator trait
   The  [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)  in Rust defines the behavior for types that can produce a sequence of  values one at a time, providing a common interface for iterating over  collections or generating streams of data.

   Some common types that implement iterators in Rust are `tuples`, `arrays`, `Vec`s and types from Rust [std::collections](https://doc.rust-lang.org/std/collections/index.html) like `VecDeque` and `HashMap`s.
   ```rust
   use std::collections::HashMap;
   
   fn main() {
       let array = [1u8, 2, 3, 4, 5];
       //Iterate over the array
       for number in array {
           println!("Number: {}", number)
       }
   
       let vector = vec![1u8, 2, 3, 4, 5, 6];
       //Iterate over the vector
       for number in vector {
           println!("Number: {}", number)
       }
   
       let mut map = HashMap::<u8, String>::new();
       map.insert(1, "Number one".to_string());
       map.insert(2, "Number two".to_string());
       //Iterate over the hashmap printing the key and value
       for (key, value) in &map {
           println!("Key: {} - Value: {}", key, value)
       }
   
       //Iterate over the keys of hashmap
       for key in map.keys() {
           println!("Key: {}", key)
       }
   
       //Iterate over the values of hashmap
       for value in map.values() {
           println!("Value: {}", value)
       }
   }
   ```

   - Implementing iterator for user defined types
     ```rust
     // Bring the trait to scope
     use std::ops::Add;
     
     fn main() {
         let community1 = Community::new("RustKe", 500);
         let community2 = Community::new("Flutter", 1500);
     
         let sum = community1 + community2;
     
         println!("Sum of all community members is: {}", sum);
     }
     
     pub struct Communities(Vec<Community>);
     
     impl Iterator for Communities {
         type Item = Community;
     
         fn next(&mut self) -> Option<Self::Item> {
             if let Some(community) = self.0.iter().next() {
                 Some(community.clone())
             } else {
                 Option::None
             }
         }
     }
     
     #[derive(Clone)]
     pub struct Community {
         name: String,
         // The number of registered members
         numbers: u16,
     }
     
     impl Community {
         fn new(name: &str, numbers: u16) -> Self {
             Self {
                 name: name.to_string(),
                 numbers,
             }
         }
     }
     
     impl Add for Community {
         type Output = u16;
     
         fn add(self, rhs: Self) -> Self::Output {
             // Since we cannot add 2 names together
             // we will add the `numbers`
             self.numbers + rhs.numbers
         }
     }
     ```

     


