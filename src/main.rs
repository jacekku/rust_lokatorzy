use std::{
    fmt::{Display, Formatter},
    vec,
};

use inquire::{error::InquireResult, validator::Validation, CustomType, Select, Text};

struct Person {
    name: String,
    produce: Vec<Product>,
}
struct Product {
    name: String,
    price: f64,
}
#[derive(Debug, Clone)]
enum MenuOption {
    AddPerson,
    AddProduce,
    PrintList,
    Exit,
}
impl MenuOption {
    const VARIANTS: &'static [MenuOption] = &[
        Self::AddProduce,
        Self::PrintList,
        Self::AddPerson,
        Self::Exit,
    ];
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
fn add_product_to_person(person_list: &mut Vec<Person>) -> Result<(), inquire::InquireError> {
    let person_name =
        Select::new("", person_list.iter().map(|p| p.name.clone()).collect()).prompt()?;

    let mut person_option: Option<&mut Person>;
    {
        person_option = None;
        for p in person_list.iter_mut() {
            if p.name.eq(&person_name.clone()) {
                person_option = Some(p);
                break;
            }
        }
    }
    Ok(match person_option {
        Some(person) => add_product(person)?,
        None => println!("No person found with that name"),
    })
}

fn add_product(person: &mut Person) -> Result<(), inquire::InquireError> {
    println!("Person found with name {}", person.name);
    let name = Text::new("Enter produce name: ").prompt()?;
    let price = CustomType::<f64>::new("How much did it cost?")
        .with_formatter(&|i| format!("{:.2} PLN", i))
        .with_error_message("Please type a valid number")
        .with_validator(|val: &f64| {
            if *val >= 0.0f64 {
                return Ok(Validation::Valid);
            }
            return Ok(Validation::Invalid("You must enter positive amount".into()));
        })
        .prompt()?;
    person.produce.push(Product { name, price });
    Ok(())
}

fn print_list(list: &Vec<Person>) {
    let mut total = 0.0f64;
    for person in list {
        println!("{}", person.name);
        for product in person.produce.iter() {
            println!(" - {} {:.2} PLN", product.name, product.price)
        }
        let person_total = person
            .produce
            .iter()
            .map(|e| e.price)
            .reduce(|acc, e| acc + e)
            .unwrap_or(0.0f64);
        println!("  Total: {:.2} PLN", person_total);
        total += person_total;
    }
    println!("Total: {:.2} PLN", total);
}

fn main() -> InquireResult<()> {
    let mut person_list: Vec<Person> = vec![];

    person_list.push(Person {
        name: "Jacek".to_string(),
        produce: vec![],
    });
    person_list.push(Person {
        name: "Magda".to_string(),
        produce: vec![],
    });

    loop {
        let ans: MenuOption = Select::new("Choice", MenuOption::VARIANTS.to_vec()).prompt()?;
        match ans {
            MenuOption::AddProduce => {
                if person_list.len() == 0 {
                    println!("No person added to list, please add one to continue");
                    continue;
                }
                add_product_to_person(&mut person_list)?;
            }
            MenuOption::PrintList => print_list(&person_list),
            MenuOption::Exit => return Ok(()),
            MenuOption::AddPerson => {
                let new_person_name = Text::new("Enter new person name").prompt()?;
                let list = &person_list;

                let exists = list.iter().find(|p| p.name.eq(&new_person_name.clone()));

                match exists {
                    Some(person) => println!("Person with name: {} already exists", person.name),
                    None => person_list.push(Person {
                        name: new_person_name,
                        produce: vec![],
                    }),
                }
            }
        }
    }
}
