pub mod mall;
pub use mall::floor::store::employee::*;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::guard::*;
pub use mall::*;

pub fn biggest_store(m: mall::Mall) -> store::Store {
    let mut max_store = Store::new("", 0, Vec::new());
    let mut max_size: u64 = 0;
    for floor in m.floors {
        for store in &floor.stores {
            if store.square_meters > max_size {
                max_store = store.clone();
                max_size = store.square_meters;
            }
        }
    }
    max_store
}
pub fn highest_paid_employee(m: mall::Mall) -> Vec<floor::store::employee::Employee> {
    let mut employees: Vec<floor::store::employee::Employee> = Vec::new();
    let mut max_salary: f64 = 0.0;
    for floor in m.floors {
        for store in &floor.stores {
            for emp in &store.employees {
                if emp.salary > max_salary {
                    max_salary = emp.salary;
                    employees.clear();
                    employees.push(emp.clone());
                } else if emp.salary == max_salary {
                    employees.push(emp.clone());
                }
            }
        }
    }
    employees
}
pub fn nbr_of_employees(m: mall::Mall) -> usize {
    let mut nbr_employees = m.guards.len();
    for floor in m.floors {
        for store in &floor.stores {
            nbr_employees += store.employees.len();
        }
    }
    nbr_employees
}
pub fn check_for_securities(m: &mut mall::Mall, guards: Vec<mall::guard::Guard>) {
    let mut total_size: usize = 0;
    for floor in &m.floors {
        total_size += floor.size_limit as usize;
    }
    let nbr_employee_needs = (total_size / 200) - m.guards.len();
    if nbr_employee_needs > 0 {
        for j in 0..nbr_employee_needs {
            mall::Mall::hire_guard(m, guards[j].clone());
        }
    }
}
pub fn cut_or_raise(m: &mut mall::Mall) {
    for floor in m.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in store.employees.iter_mut() {
                let working_hours = employee.working_hours.1 - employee.working_hours.0;
                if working_hours > 10 {
                    employee.raise(employee.salary * 0.1)
                } else {
                    employee.cut(employee.salary * 0.1)
                };
            }
        }
    }
}