use crate::model::foreman::Foreman;

pub struct System {}

impl System {

    pub fn start(miners: i32, zones: i32) {
        println!();

        let mut foreman: Foreman = Foreman::new(zones);
        foreman.hire_miners(miners);
        foreman.start_mining();

    }

}
