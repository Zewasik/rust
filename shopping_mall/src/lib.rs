use mall::floor::store::Store;
pub mod mall;
pub use crate::mall::floor;
pub use crate::mall::floor::store;
use crate::mall::floor::store::employee::Employee;
use crate::mall::guard::Guard;
use crate::mall::Mall;

pub fn biggest_store(m: Mall) -> Store {
    let mut biggest = Store {
        name: "val".to_string(),
        square_meters: 0,
        employees: Vec::new(),
    };
    for floor in m.floors {
        for store in floor.stores {
            if store.square_meters > biggest.square_meters {
                biggest = store
            }
        }
    }
    return biggest;
}
pub fn highest_paid_employee(m: Mall) -> Vec<Employee> {
    let mut hp_empl: Vec<Employee> = Vec::new();
    for floor in m.floors {
        for store in floor.stores {
            for empl in store.employees {
                let temp = empl.clone();
                let vec_len = hp_empl.len();
                if vec_len == 0 {
                    hp_empl.push(temp)
                } else if vec_len > 0 && hp_empl[vec_len - 1].salary < temp.salary {
                    hp_empl[vec_len - 1] = temp
                } else if hp_empl[vec_len - 1].salary == temp.salary {
                    hp_empl.push(temp)
                }
            }
        }
    }
    return hp_empl;
}
pub fn nbr_of_employees(m: Mall) -> usize {
    let mut temp: usize = 0;
    for floor in m.floors {
        for store in floor.stores {
            temp += store.employees.len();
        }
    }
    temp += m.guards.len();
    return temp;
}
pub fn fire_old_securities(m: &mut Mall) {
    let mut counter = 0;
    let mut guard_index: Vec<usize> = Vec::new();
    for guard in &m.guards {
        if guard.age >= 50 {
            guard_index.push(counter)
        }
        counter += 1;
    }
    for index in guard_index {
        m.guards.remove(index);
    }
}
pub fn check_for_securities(m: &mut Mall, mut guards: Vec<Guard>) {
    let mut area = 0;
    for floor in &m.floors {
        area += floor.size_limit;
    }
    while area / m.guards.len() as u64 > 200 {
        let guard1 = guards.swap_remove(0);
        m.guards.push(guard1);
    }
}
pub fn cut_or_raise(m: &mut Mall) {
    for floor in m.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for mut empl in store.employees.iter_mut() {
                if empl.working_hours.1 - empl.working_hours.0 > 10 {
                    empl.salary += empl.salary / 10.0
                } else {
                    empl.salary -= empl.salary / 10.0
                }
            }
        }
    }
}
