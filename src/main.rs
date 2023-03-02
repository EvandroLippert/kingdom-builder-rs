pub enum Categorie {
    King,
    Noble,
    Warrior,
    Peasant,
}

#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female,
}

pub struct Kingdom {
    pub name: String,
    pub year: u8,
    pub citzen: Vec<Citzen>,
    pub kings: Vec<Citzen>,
    pub nobles: Vec<Citzen>,
    pub warriors: Vec<Citzen>,
    pub peasants: Vec<Citzen>,
}

pub struct Citzen {
    pub categorie: Categorie,
    pub gender: Gender,
    pub age: u8,
    pub name: String,
    pub is_preagnant: bool,
}

pub trait KingdomSummary {
    fn summarize(&self);

    fn total_of_citzens(&self) -> u8;
}

pub trait KingdomHandler {
    fn new_year(&mut self);
    fn generate_citzen(categorie: Categorie, gender: Gender, age: u8, name: String) -> Citzen;
}

pub trait CitzenHandler {
    fn make_a_child(cit_1: &mut Citzen, cit_2: &mut Citzen);

    //fn born(&self) -> Citzen;
    //fn die(&self) -> bool;
}

impl CitzenHandler for Citzen {
    fn make_a_child(cit_1: &mut Citzen, cit_2: &mut Citzen) {
        if &cit_1.gender != &cit_2.gender {
            if &cit_2.gender == &Gender::Female {
                *&mut cit_2.is_preagnant = true;
                println!("Congrats {:?} is preagnant", &cit_2.name);
            } else {
                *&mut cit_1.is_preagnant = true;
                println!("Congrats {:?} is preagnant", &cit_1.name);
            }
        }
    }
}

impl KingdomSummary for Kingdom {
    fn summarize(&self) {
        println!("Currently our kingdom of")
    }
    fn total_of_citzens(&self) -> u8 {
        return 1;
    }
}
impl KingdomHandler for Kingdom {
    fn new_year(&mut self) {
        self.year += 1;
        println!("Happy new year of {}", self.year);
    }

    fn generate_citzen(categorie: Categorie, gender: Gender, age: u8, name: String) -> Citzen {
        let citzen = Citzen {
            categorie: categorie,
            gender: gender,
            age: age,
            name: name,
            is_preagnant: false,
        };

        citzen
    }
}

fn main() {
    let mut great_kingdom = Kingdom {
        name: String::from("Brazil"),
        citzen: vec![],
        year: 0,
        kings: vec![],
        nobles: vec![],
        warriors: vec![],
        peasants: vec![],
    };

    let mut joseph =
        Kingdom::generate_citzen(Categorie::King, Gender::Male, 18, String::from("Joseph"));

    let mut josephine = Kingdom::generate_citzen(
        Categorie::King,
        Gender::Female,
        18,
        String::from("Josephine"),
    );

    <Citzen as CitzenHandler>::make_a_child(&mut joseph, &mut josephine);
    println!("Josephine is preagnant: {:?}", josephine.is_preagnant);
    great_kingdom.new_year();
}
