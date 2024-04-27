use std::collections::HashMap;

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        self.departments
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(name.to_string());
    }

    fn list_department(&self, department: &str) {
        if let Some(employees) = self.departments.get(department) {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("Employees in {}: {:?}", department, sorted_employees);
        } else {
            println!("No employees found in {}.", department);
        }
    }

    fn list_all(&self) {
        let mut all_employees: Vec<String> = Vec::new();

        for (department, employees) in &self.departments {
            for employee in employees {
                all_employees.push(format!("{} ({})", employee, department));
            }
        }

        all_employees.sort();
        println!("All employees:");
        for employee in all_employees {
            println!("{}", employee);
        }
    }
}

fn main() {
    let mut company = Company::new();

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Add an employee to a department");
        println!("2. List employees in a department");
        println!("3. List all employees in the company");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number from 1 to 4.");
                continue;
            }
        };

        match choice {
            1 => {
                let mut name = String::new();
                let mut department = String::new();
                println!("Enter employee name:");
                std::io::stdin().read_line(&mut name).expect("Failed to read line");
                println!("Enter department name:");
                std::io::stdin().read_line(&mut department).expect("Failed to read line");
                company.add_employee(name.trim(), department.trim());
                println!("Employee added to department.");
            }
            2 => {
                let mut department = String::new();
                println!("Enter department name:");
                std::io::stdin().read_line(&mut department).expect("Failed to read line");
                company.list_department(department.trim());
            }
            3 => {
                company.list_all();
            }
            4 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please enter a number from 1 to 4."),
        }
    }
}

