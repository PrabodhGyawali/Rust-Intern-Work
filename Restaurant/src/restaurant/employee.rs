
trait Work {
    fn work(&self) -> String;
}

pub struct Employee {
    id: u32,
    wage: u32,
}

struct Chef {
    employee: Employee,
    skills: f32,
}

struct Waiter {
    employee: Employee,
    skills: f32,
}

impl Work for Chef {
    fn work(&self, order: Order) -> String {
        format!("The chef with ID {} is cooking order with ID {}", self.employee.id);
    }
}

impl Work for Waiter {
    fn work(&self) -> String {
        format!("The waiter with ID {} is serving cusomer")
    }
}