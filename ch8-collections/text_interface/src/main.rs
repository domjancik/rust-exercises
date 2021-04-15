use std::collections::HashMap;
use std::io;
use std::str::SplitWhitespace;

// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department
// in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

#[derive(Debug)]
enum CommandError {
    ActionMissing,
    NameMissing,
    MalformedCommand,
    DepartmentMissing,
    UnknownCommand,
    NoSuchDepartment
}

fn main() {
    let mut departments = HashMap::new();

    println!("Welcome to Company Directory v199.3021 by SOPHTWARE");
    println!("Bringing you the most sophisticated software since the dawn of the third era");
    println!("");
    println!("Supported commands:");
    println!("Add, List, All, Exit");

    loop {

        let mut command = String::new();
    
        io::stdin().read_line(&mut command).expect("Failed to read");
        let mut words = command.split_whitespace();
        
        let action = words.next();
    
        let res = match action {
            Some(a) => match a {
                "Add" => add(&mut departments, &mut words),
                "List" => list_department(&mut departments, &mut words),
                "All" => list_all(&mut departments),
                "Exit" => break,
                _ => {
                    Err(CommandError::UnknownCommand)
                }
            },
            None => Err(CommandError::ActionMissing)
        };

        match res {
            Ok(_) => (),
            Err(e) => println!("Failed: {:?}", e)
        }
    
        // Uncomment to see contents after each step
        // println!("{:?}", departments);
    }
}

fn add(map: &mut HashMap<String, Vec<String>>, token_iterator: &mut SplitWhitespace) -> Result<(), CommandError> {
    let name = match token_iterator.next() {
        Some(v) => v,
        None => return Err(CommandError::NameMissing),
    };

    match token_iterator.next() {
        Some("to") => (),
        _ => return Err(CommandError::MalformedCommand),
    };

    let department = match token_iterator.next() {
        Some(v) => v,
        None => return Err(CommandError::DepartmentMissing),
    };

    let dep = map.entry(String::from(department)).or_insert(vec![]);
    dep.push(String::from(name));

    Ok(())
}

fn print_department(department: &str, people: &Vec<String>) {
    let mut sorted_people = people.to_vec();
    sorted_people.sort();
    
    println!("{}:", department);
    for person in sorted_people {
        println!("- {}", person);
    }
}

fn list_department(map: &HashMap<String, Vec<String>>, token_iterator: &mut SplitWhitespace) -> Result<(), CommandError> {
    let department = match token_iterator.next() {
        Some(v) => v,
        None => return Err(CommandError::DepartmentMissing),
    };

    let dep = map.get(department);

    match dep {
        Some(people) => {
            print_department(department, people);
            Ok(())
        },
        None => return Err(CommandError::NoSuchDepartment)
    }
}

fn list_all(map: &HashMap<String, Vec<String>>) -> Result<(), CommandError> {
    let departments = map.keys();
    let mut sorted_keys: Vec<&String> = departments.collect();
    sorted_keys.sort();
    for key in sorted_keys {
        let people = map.get(key).expect("All keys should be valid");
        print_department(key, people)
    }

    Ok(())
}