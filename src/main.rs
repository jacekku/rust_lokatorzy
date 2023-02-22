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

#[derive(Clone)]
struct Product {
    name: String,
    price: f64,
}
#[derive(Debug, Clone)]
enum MenuOption {
    AddPerson,
    AddProduce,
    ModifyProduct,
    DeleteProduct,
    PrintList,
    Exit,
}
impl MenuOption {
    const VARIANTS: &'static [MenuOption] = &[
        Self::AddProduce,
        Self::PrintList,
        Self::AddPerson,
        Self::ModifyProduct,
        Self::DeleteProduct,
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
                let person_name = get_person_name(&person_list)?;

                let product = get_product_data()?;
                add_product_to_person(&mut person_list, person_name, product);
            }
            MenuOption::AddPerson => {
                let new_person_name = get_new_person_name()?;
                add_person(&mut person_list, new_person_name);
            }
            MenuOption::ModifyProduct => {}
            MenuOption::DeleteProduct => {
                let person_name = get_person_name(&person_list)?;
                let products = person_list.get(&person_name);
                match products {
                    Some(products) => {
                        if products.len() == 0 {
                            println!("Product list is empty");
                            continue;
                        }
                        let product_to_delete = Select::new(
                            "Which product to delete",
                            products
                                .iter()
                                .map(|p| format!("{} {:.2} PLN", p.name.clone(), p.price.clone()))
                                .collect(),
                        )
                        .prompt()?;

                        let product_index =
                            products.iter().position(|p| !p.name.eq(&product_to_delete));

                        let mut new_products = products.to_vec();

                        match product_index {
                            Some(index) => {
                                new_products.remove(index);
                            }
                            None => {}
                        }
                        person_list.insert(person_name, new_products);
                    }
                    None => todo!(),
                }
            }
            MenuOption::PrintList => print_list(&person_list),
            MenuOption::Exit => return Ok(()),
        }
    }
}
