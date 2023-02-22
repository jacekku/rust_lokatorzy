use std::collections::HashMap;

use crate::Product;

pub(crate) fn print_list(list: &HashMap<String, Vec<Product>>) {
    let mut total = 0.0f64;
    for (name, products) in list {
        println!("{}", name);
        for product in products {
            println!(" - {} {:.2} PLN", product.name, product.price)
        }
        let person_total = products
            .iter()
            .map(|e| e.price)
            .reduce(|acc, e| acc + e)
            .unwrap_or(0.0f64);
        println!("  Total: {:.2} PLN", person_total);
        total += person_total;
    }
    println!("Total: {:.2} PLN", total);
}
