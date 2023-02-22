mod add_person;
mod add_product;
mod inquire_adapter;
mod print_list;

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    vec,
};

use add_person::add_person;
use add_product::add_product_to_person;
use inquire::{error::InquireResult, Select};
use inquire_adapter::{get_new_person_name, get_person_name, get_product_data};
use print_list::print_list;

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

fn main() -> InquireResult<()> {
    let mut person_list: HashMap<String, Vec<Product>> = HashMap::new();

    person_list.insert("Jacek".to_string(), vec![]);
    person_list.insert("Magda".to_string(), vec![]);

    loop {
        let ans: MenuOption = Select::new("Choice", MenuOption::VARIANTS.to_vec()).prompt()?;
        match ans {
            MenuOption::AddProduce => {
                if person_list.len() == 0 {
                    println!("No person added to list, please add one to continue");
                    continue;
                }
                let person_name = get_person_name(&person_list)?;

                let product = get_product_data()?;
                add_product_to_person(&mut person_list, person_name, product);
            }
            MenuOption::AddPerson => {
                let new_person_name = get_new_person_name()?;
                add_person(&mut person_list, new_person_name);
            }
            MenuOption::PrintList => print_list(&person_list),
            MenuOption::Exit => return Ok(()),
        }
    }
}
