use rand::Rng;

#[derive(Debug)]
pub struct Customer {
    pub id: u32,
    pub is_waiting: bool,
    pub size: u8,
    pub nice: u8,   // for tipping algorithm && Feedback 
}

impl Customer {
    pub fn new(id: u32, size: u8, nice: u8) -> Self {
        Self {
            id, 
            name, 
            is_waiting: true, 
            size,
            nice,
        }
    }
    pub fn enter_restaurant(&self) {
        println!("Customer {} has entered the restaurant.", self.id);
        // Put the Customer into waiting state
        let wait_time = self.calculate_wait_time();
        // Implement a command line bar loading until wait time is over
    }

    fn calculate_wait_time(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let base_wait = rng.gen_range(5..=10);
        let nice_adjustment = self.nice as u32 / 2;
        base_wait.saturating_sub(nice_adjustment);
    } 
}

//// TODO
pub fn take_order(&mut self) {}

pub fn ask_for_water_refill() {}
// -10 to 10 -> for obtaining of tips

pub fn make_payment() {}

pub fn give_tip() {}    // tip waiter --> restuarant takes tips LOL
// Affected by nice level and Waiter Skill level

pub fn give_feedback() {}   // affects restaurant rating
// Affected by Waiter Skill level & Chef Skill level

