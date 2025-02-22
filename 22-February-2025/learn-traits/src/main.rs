fn main() {
    // Charles
    // Linet
    // Khalid

    let charles = FormCollection {
        name: "Charles".to_string(),
        number_of_rooms: NoOfRooms::TwoBedroom,
        title_deed: "NC-25".to_string(),
        r#type: Type::Villa,
        color: Color::Cream,
        decor: Decor::African,
        dependants: 2,
        street: "Kenyatta Avenue".to_string(),
        county: County::Nairobi,
    };

    let linet = FormCollection {
        name: "Linet".to_string(),
        number_of_rooms: NoOfRooms::ThreeBedroom,
        title_deed: "NC-26".to_string(),
        r#type: Type::Bungalow,
        color: Color::Red,
        decor: Decor::Italian,
        dependants: 2,
        street: "Kenyatta Avenue".to_string(),
        county: County::Nairobi,
    };

    let khalid = FormCollection {
        name: "Khalid".to_string(),
        number_of_rooms: NoOfRooms::OneBedroom,
        title_deed: "NC-27".to_string(),
        r#type: Type::Penthouse,
        color: Color::Blue,
        decor: Decor::German,
        dependants: 1,
        street: "Mama Ngina Street".to_string(),
        county: County::Nairobi,
    };

    println!("{:?}", Participant::default());

    let check_default_webapp = WebApp::default();
    println!("{:?}", check_default_webapp);

    let john_realtor = Participant::default();
    let mercy_county_official = Participant::CountyOfficial;
    let james = Participant::InteriorDesigner;

    let rooms = NoOfRooms::OneBedroom;
    // Show implementation for std::fmt::Debug using `:?`
    println!("{:?}", rooms);
    // Show implementation for std::fmt::Display
    println!("{}", rooms);
}

#[derive(Debug, Default)]
struct WebApp(Participant);

impl WebApp {
    fn new(participant: Participant) -> Self {
        Self(participant)
    }

    fn screen(&self, form: &FormCollection) {
        if self.0 == Participant::Realtor {
            println!(
                "REALTOR SCREEN:
            name: {},
            number_of_rooms: {:?},
            title_deed: {},
            r#type: {:?},
            color: {:?},
            decor: {:?},
            dependants: {},
            street: {},
            county: {:?},
            ",
                &form.name(),
                &form.number_of_rooms(),
                &form.title_deed(),
                &form.r#type(),
                &form.color(),
                &form.decor(),
                &form.dependants(),
                &form.street_name(),
                &form.county(),
            )
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Participant {
    CountyOfficial,
    #[default]
    Realtor,
    InteriorDesigner,
}

// impl Default for Participant {
//     fn default() -> Self {
//         Participant::Realtor
//     }
// }

#[derive(Debug)]
struct FormCollection {
    name: String,
    number_of_rooms: NoOfRooms,
    title_deed: String,
    r#type: Type,
    color: Color,
    decor: Decor,
    dependants: u8,
    street: String,
    county: County,
}

impl House for FormCollection {
    fn title_deed(&self) -> String {
        self.title_deed.clone()
    }

    fn r#type(&self) -> Type {
        self.r#type
    }

    fn color(&self) -> Color {
        self.color
    }

    fn decor(&self) -> Decor {
        self.decor
    }

    fn number_of_rooms(&self) -> NoOfRooms {
        self.number_of_rooms
    }
}

impl Person for FormCollection {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Location for FormCollection {
    fn county(&self) -> County {
        County::Nairobi
    }

    fn street_name(&self) -> String {
        self.street.to_string()
    }
}

trait House: Person + Location {
    fn title_deed(&self) -> String;

    fn r#type(&self) -> Type;

    fn color(&self) -> Color;

    fn decor(&self) -> Decor;

    fn number_of_rooms(&self) -> NoOfRooms;
}
trait Person {
    fn name(&self) -> String;

    fn dependants(&self) -> u8 {
        2_u8
    }
}

trait Location {
    fn street_name(&self) -> String;

    fn county(&self) -> County;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum NoOfRooms {
    OneBedroom,
    TwoBedroom,
    ThreeBedroom,
}

impl std::fmt::Display for NoOfRooms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The number of rooms are: {:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Decor {
    African,
    German,
    Italian,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Type {
    Apartment,
    Villa,
    Bungalow,
    Penthouse,
    Mansion,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
    Cream,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum County {
    Nairobi,
    Kiambu,
    Kajiado,
    Mombasa,
}
