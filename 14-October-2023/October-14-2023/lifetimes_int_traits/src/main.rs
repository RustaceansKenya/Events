use std::borrow::Cow;

fn main() {
    let foo = Foo {
        data: String::from("Mr Kamau"),
    };

    foo.has_alien_dna();
}

struct Foo {
    data: String,
}

impl IsHuman for Foo {
    fn has_skeleton() -> bool {
        true
    }

    fn has_alien_dna(&self) -> &'static dyn FirstContact {
        Box::new(self)
    }
}

trait IsHuman {
    fn has_skeleton() -> bool;

    fn has_alien_dna(&self) -> &'static dyn FirstContact;
}

pub trait FirstContact {
    fn greeting(&self) -> Cow<str>;
}

impl FirstContact for Foo {
    fn greeting(&self) -> Cow<str> {
        self.data.clone().into()
    }
}
