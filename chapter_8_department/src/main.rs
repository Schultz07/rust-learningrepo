use std::{collections::HashMap, io};

fn main() {
    let mut department_employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut option = String::new();
        println!("Welcome to the department choosing department!");
        println!("");
        println!("");
        println!("");
        println!("1. Add new employee");
        println!("2. List employees in a department");
        println!("3. List all employees by department");
        println!("4. Quit");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        match option {
            1 => add_new_employee(&mut department_employees),
            2 => list_employees_in_department(&department_employees),
            3 => list_all_employees_by_department(&department_employees),
            4 => return,
            _ => continue,
        }
    }
}

fn add_new_employee(department_employees: &mut HashMap<String, Vec<String>>) {
    println!("Tell us the employee's name:");
    let mut employee_name = String::new();

    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");

    println!("Tell us which department he'll be joining:");
    let mut employee_department = String::new();

    io::stdin()
        .read_line(&mut employee_department)
        .expect("Failed to read line");

    let entry = department_employees
        .entry(employee_department.trim().to_string())
        .or_insert(vec![]);
    entry.push(employee_name.trim().to_string());
    entry.sort();
}

fn list_employees_in_department(department_employees: &HashMap<String, Vec<String>>) {
    println!("What department would you like to consult?");
    let department_list: Vec<&String> = department_employees.keys().collect();
    println!("{department_list:?}");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to read line");
    let employees = department_employees.get(&department_name.trim().to_string());

    match employees {
        Some(e) => println!("{e:?}"),
        None => println!("Department not found"),
    }
    wait_for_input();
}

fn list_all_employees_by_department(department_employees: &HashMap<String, Vec<String>>) {
    let mut department_list: Vec<&String> = department_employees.keys().collect();
    department_list.sort();
    for department in department_list {
        if let Some(employees) = department_employees.get(department) {
            println!("{department}: {employees:?}");
        }
    }
    wait_for_input();
}

fn wait_for_input() {
    println!("Press any key...");
    let mut wait_for_input = String::new();
    io::stdin()
        .read_line(&mut wait_for_input)
        .expect("Failed to read line");
}
