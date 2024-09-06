use inquire::{
    formatter::MultiCountFormatter, list_option::ListOption, validator::Validation, MultiCount,
};

fn main() {
    let options = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let formatter: MultiCountFormatter<'_, &str> = &|a| format!("{} different fruits", a.len());

    let ans = MultiCount::new("Select the fruits for your shopping list:", options)
        .with_formatter(formatter)
        .prompt();

    match ans {
        Ok(_) => println!("I'll get right on it"),
        Err(_) => println!("The shopping list could not be processed"),
    }
}
