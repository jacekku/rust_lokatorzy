use std::collections::HashMap;

use crate::Product;

pub(crate) fn add_product_to_person(
    person_list: &mut HashMap<String, Vec<Product>>,
    person_name: String,
    product: Product,
) {
    let person_option = person_list.get_mut(&person_name);

    match person_option {
        Some(person) => person.push(product),
        None => println!("No person found with that name"),
    };
}
