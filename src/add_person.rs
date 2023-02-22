use std::collections::HashMap;

use crate::Product;

pub(crate) fn add_person(person_list: &mut HashMap<String, Vec<Product>>, new_person_name: String) {
    let exists = person_list.contains_key(&new_person_name);
    if exists {
        println!("Person with name: {} already exists", new_person_name);
    } else {
        person_list.insert(new_person_name, vec![]);
    };
}
