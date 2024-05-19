pub mod mall;
pub use mall::floor::store::employee::*;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::guard::*;
pub use mall::*;

pub fn biggest_store(mall: Mall) -> Store {
    let mut max = 0;
    let mut max_store = Store::new("", 0, Vec::new());
    let floors = mall.floors;
    for floor in floors {
        let stores = floor.stores;
        for store in stores.iter() {
            if store.square_meters > max {
                max = store.square_meters;
                max_store = store.clone();
            }
        }
    }
    max_store.clone()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut max_salaire = 0.0;
    let mut higgest = Employee::new("", 0, 0, 0, 0.0);
    let floors = mall.floors;
    let mut result = Vec::new();
    for floor in floors {
        let stores = floor.stores;
        for store in stores.iter() {
            let employees = store.employees.clone();
            for employee in employees.iter() {
                if employee.salary > max_salaire {
                    max_salaire = employee.salary;
                    higgest = employee.clone();
                    result.clear();
                    result.push(higgest)
                }else if  employee.salary == max_salaire{
                    higgest = employee.clone();
                    result.push(higgest)
                }
            }
        }
    }
    result
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut nbr_employee = 0;
    let floors = mall.floors.clone();
    let nbr_guards = mall.guards.len();
    for floor in floors {
        let stores = floor.stores.clone();
        for store in stores.iter() {
            let employees = store.employees.clone();
            nbr_employee += employees.len().clone();
        }
    }
    nbr_employee.clone()+nbr_guards
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let mut size = 0;
    for floor in mall.floors.iter() {
        size += floor.size_limit;
    }
    let normal = size / 200;
    let i = normal as usize - mall.guards.len();
    if i > 0 {
        for j in 0..i {
            mall::Mall::hire_guard(mall, guards[j].clone());
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    let mut floors = mall.floors.clone();
    for floor in floors.iter_mut() {
        let mut stores = floor.stores.clone();
        for store in stores.iter_mut() {
            let mut employees = store.employees.clone();
            for employee in employees.iter_mut() {
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee::Employee::raise(employee,employee.salary * 0.1);
                } else {
                    employee::Employee::cut(employee,employee.salary * 0.1); 
                }
            }
            store.employees= employees;
        }
        floor.stores=stores;
    }
    mall.floors=floors;
}