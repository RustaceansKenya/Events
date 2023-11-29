use std::borrow::Cow;

fn main() {}

#[derive(Debug, Default)]
struct PoS<'a, 'b, T> {
    name: &'static str,
    client: &'a str,
    destination: Cow<'b, str>,
    shipping_id: Option<&'b T>,
}

impl<'a, 'b, T: Default> PoS<'a, 'b, T> {
    pub fn new<'a>() -> Self {
        PoS::default()
    }

    pub fn add_name<'a>(&mut self, name: &'static str) -> &mut Self {
        self.name = name;

        self
    }

    pub fn add_client<'a>(&mut self, client: &'a str) -> &mut Self {
        self.client = client;

        self
    }

    pub fn add_destination<'a>(&mut self, destination: &'a str) -> &mut Self {
        self.destination = Cow::Borrowed(destination);

        self
    }

    pub fn add_shipping_id(&mut self, shipping_id: &'a T) -> &mut Self {
        self.shipping_id = Some(shipping_id);

        self
    }

    pub fn client(&self) -> &str {
        self.client
    }

    /// SInce lifetime of `'c` is only used in build function it can
    /// be defined for this scope only
    pub fn build<'c>(&self) -> Result<&str, &str> {
        let shipping_id = match self.shipping_id {
            Some(id) => id,
            None => return Err("Shipping ID is needed"),
        };

        //Using `&self.destination` derefences the Cow in a way that it can be added to a string using `+`
        Ok(&(String::from(self.client)
            + ": "
            + &self.destination
            + ": "
            + shipping_id
            + "-> Shop: "
            + self.name))
    }
}

pub fn collect_string(values: &[&str; 3]) -> &str {
    values.iter().collect::<&str>()
}
