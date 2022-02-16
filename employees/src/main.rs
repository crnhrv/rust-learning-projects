use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the Company records");

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    employees.insert("Sales".to_string(), vec![]);
    employees.insert("Engineering".to_string(), vec![]);

    loop {
        println!("What would you like to do?");
        println!("A: Add a new employee");
        println!("R: See the record of employees");
        println!("X: Exit the program");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let selected_option = input.trim().to_lowercase();
                if selected_option == "a" {
                    add_employee(&mut employees);
                } else if selected_option == "r" {
                    see_record(&employees);
                } else if selected_option == "x" {
                    std::process::exit(0)
                } else {
                    println!("Please press either A, R or X to quit");
                    continue;
                }
            }
            Err(_) => {
                println!("Please press either A, R or X to quit");
                continue;
            }
        };
    }
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>) {
    println!("Please select or enter the name of the department you wish to add the employee to");
    
    let department = get_department(&employees);

    println!("Please enter the name of the employee you wish to add");
    let mut employee_input = String::new();
    io::stdin().read_line(&mut employee_input).expect("Invalid input");

    match employees.get(&department) {
        Some(vec) => {
            let mut employee_list = vec.clone();
            employee_list.push(employee_input.trim().to_string());
            employees.insert(department, employee_list);
        },
        None => {
            employees.insert(department, vec![employee_input.trim().to_string()]);
        } 
    }
    println!("-----------------------------------------------------------")

}

fn see_record(employees: &HashMap<String, Vec<String>>) {
    println!("Please select or enter the name of the department you wish to see the records for");
    let department = get_department(&employees);

    println!("-----------------------------------------------------------");

    match employees.get(&department) {
        Some(vec) => {
            println!("Record of employees for {} department", department.to_lowercase());
            let mut employee_list = vec.clone();
            employee_list.sort();
            for (i, employee) in employee_list.iter().enumerate() {
                println!("{}. {} [{}]", i+1, employee, department);                
            }            
        },
        None => {
            println!("That department could not be found, please try again")
        } 
    }

    println!("-----------------------------------------------------------");
    let mut input_wait = String::new(); 
    io::stdin().read_line(&mut input_wait).expect("Press enter to continue");
}

fn get_department(employees: &HashMap<String, Vec<String>>) -> String {

    let mut departments = HashMap::new();
    for (i, k) in employees.keys().enumerate() {
        println!("{}: {}", i + 1, k);
        departments.insert(i + 1, k);
    }

    let mut department_input = String::new();
    io::stdin().read_line(&mut department_input).expect("Invalid input");

    let department = match department_input.trim().parse::<usize>() {
        Ok(val) => match departments.get(&val) {
                Some(val) => val.to_string(),
                None => department_input.trim().to_string(),
            },         
        Err(_) => department_input.trim().to_string()
    };

    return department;
}