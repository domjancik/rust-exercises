use std::collections::HashMap;
use std::io;
use std::str::SplitWhitespace;

// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department
// in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

fn main() {
    let mut departments = HashMap::new();

    loop {

        let mut command = String::new();
    
        io::stdin().read_line(&mut command).expect("Failed to read");
        let mut words = command.split_whitespace();
        
        let action = words.next();
    
        match action {
            Some(a) => match a {
                "Add" => add(&mut departments, &mut words),
                _ => println!("Unknown command, try Add <name> to <department>")
            },
            None => println!("No action provided")
        }
    
        println!("{:?}", departments);
    }
}

fn add(map: &mut HashMap<String, Vec<String>>, token_iterator: &mut SplitWhitespace) {
    let name = token_iterator.next().expect("No name provided");
    let to = token_iterator.next().expect("No direction provided");
    if !to.eq("to") {
        panic!("Malformed command (use to)")
    }

    let department = token_iterator.next().expect("No department provided");

    let dep = map.entry(String::from(department)).or_insert(vec![]);
    dep.push(String::from(name));
}