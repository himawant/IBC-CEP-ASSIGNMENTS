use std::collections::HashMap;
use std::io;

// Define the Employee struct
struct Employee {
    employee_id: u32,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

// Function to retrieve employee details by employee ID
fn get_employee_by_id(id: u32, employees: &HashMap<u32, Employee>) -> Option<&Employee> {
    employees.get(&id)
}

// Function to retrieve employees with the same age
fn get_employees_by_age(age: u32, employees: &HashMap<u32, Employee>) -> Vec<&Employee> {
    employees.values().filter(|&e| e.age == age).collect()
}

fn main() {
    let mut employees: HashMap<u32, Employee> = HashMap::new();

    loop {
        println!("Enter employee details:");
        println!("Employee ID:");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read line");
        let id: u32 = id.trim().parse().expect("Invalid input");

        println!("Employee Name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        println!("Email:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");
        let email = email.trim().to_string();

        println!("Age:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read line");
        let age: u32 = age.trim().parse().expect("Invalid input");

        println!("Phone Number:");
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");
        let phone_number = phone_number.trim().to_string();

        let employee = Employee {
            employee_id: id,
            employee_name: name,
            email,
            age,
            phone_number,
        };

        employees.insert(id, employee);

        println!("Do you want to add another employee? (yes/no)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim() != "yes" {
            break;
        }
    }

    println!("Enter employee ID to get details:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");
    let id: u32 = id.trim().parse().expect("Invalid input");

    if let Some(employee) = get_employee_by_id(id, &employees) {
        println!("Employee Details:");
        println!("Name: {}", employee.employee_name);
        println!("Email: {}", employee.email);
        println!("Age: {}", employee.age);
        println!("Phone Number: {}", employee.phone_number);
    } else {
        println!("Employee not found");
    }

    println!("Enter age to get employees with the same age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Invalid input");

    let employees_with_same_age = get_employees_by_age(age, &employees);
    if !employees_with_same_age.is_empty() {
        println!("Employees with the same age:");
        for employee in employees_with_same_age {
            println!("Name: {}", employee.employee_name);
            println!("Email: {}", employee.email);
            println!("Age: {}", employee.age);
            println!("Phone Number: {}", employee.phone_number);
            println!("------");
        }
    } else {
        println!("No employees with the same age found");
    }
}