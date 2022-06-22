use std::collections::HashMap;

fn main() {
    let mut hmap = HashMap::new();
    hmap.insert("name".to_string(), "Jeff".to_string());

    let names = vec![
        "Jeff".to_string(),
        "Jameson".to_string(),
        "Nathan".to_string(),
    ];

    let ages = vec![34, 3, 1];

    let people: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    // fn print_age(name: String) {
    //     let age: i32 = people.get(&name);
    //     println!("{} is {} years old", name, age);
    // }

    // for name in names {
    //     print_age(name);
    // }
    for (key, value) in &people {
        println!("{} is {} years old", key, value);
    }

    let my_age = people.get(&"Jeff".to_string());
    if let Some(num) = my_age {
        println!("I'm {} years old", num);
    }

    // Given a list of integers,
    // use a vector and return the median
    // (when sorted, the value in the middle position)
    // and mode (the value that occurs most often;
    // a hash map will be helpful here) of the list.
    let mut num_list = vec![10, 3, 30, 4, 10, 78, 3, 20, 3, 33];

    num_list.sort();

    let median_idx = num_list.len() / 2;
    let median = num_list[median_idx];
    println!("The median is {}", median);

    // Convert strings to pig latin.
    // The first consonant of each word is moved
    // to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay”
    // added to the end instead (“apple” becomes “apple-hay”).
    //  Keep in mind the details about UTF-8 encoding!

    // Using a hash map and vectors,
    // create a text interface to
    // allow a user to add employee names
    // to a department in a company.
    // For example, “Add Sally to Engineering”
    // or “Add Amir to Sales.” Then let the user retrieve
    // a list of all people in a department or all
    // people in the company by department, sorted alphabetically.
}
