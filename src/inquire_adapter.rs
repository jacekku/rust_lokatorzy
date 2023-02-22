use std::collections::HashMap;

use inquire::{validator::Validation, CustomType, InquireError, Select, Text};

use crate::Product;

pub(crate) fn get_product_data() -> Result<Product, InquireError> {
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
    Ok(Product { name, price })
}

pub(crate) fn get_person_name(
    person_list: &HashMap<String, Vec<Product>>,
) -> Result<String, InquireError> {
    let person_name = Select::new("", person_list.keys().map(|p| p.clone()).collect()).prompt()?;
    Ok(person_name)
}

pub(crate) fn get_new_person_name() -> Result<String, InquireError> {
    let new_person_name = Text::new("Enter new person name").prompt()?;
    Ok(new_person_name)
}
