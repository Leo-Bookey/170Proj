use std::cmp;
use std::f64;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Task {
    id: u32,
    deadline: u64,
    duration: u64,
    perfect_reward: f64,
}

impl Task {
    pub fn new(id: u32, deadline: u64, duration: u64, perfect_reward: f64) -> Task {
        Task {
            id,
            deadline,
            duration,
            perfect_reward,
        }
    }

    pub fn new_null() -> Task {
        Task {
            id: 0,
            deadline: 0,
            duration: 0,
            perfect_reward: 0.0,
        }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_deadline(&self) -> u64 {
        self.deadline
    }

    pub fn get_duration(&self) -> u64 {
        self.duration
    }

    pub fn get_perfect_reward(&self) -> f64 {
        self.perfect_reward
    }

    pub fn get_late_penalty(&self, mut mins_past: i64) -> f64 {
        mins_past = cmp::max(mins_past, 0);
        self.get_perfect_reward() * f64::exp(-0.0170*mins_past as f64)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MaxTask {
    max_seq: Vec<Task>,
    max_cost: f64,
    opt_stat: f64,
}

impl MaxTask {
    pub fn new(max_seq: Vec<Task>, max_cost: f64, opt_stat: f64) -> MaxTask {
        MaxTask {
            max_seq: max_seq,
            max_cost: max_cost,
            opt_stat: opt_stat,
        }
    }

    pub fn new_empty() -> MaxTask {
        MaxTask {
            max_seq: Vec::new(),
            max_cost: 0.0,
            opt_stat: 0.0,
        }
    }

    pub fn get_max_seq(&self) -> &Vec<Task> {
        &self.max_seq
    }

    pub fn get_max_cost(&self) -> f64 {
        self.max_cost
    }

    pub fn get_opt_stat(&self) -> f64 {
        self.opt_stat
    }
}

impl PartialOrd for MaxTask {
    fn partial_cmp(&self, other: &MaxTask) -> Option<std::cmp::Ordering> {
        self.opt_stat.partial_cmp(&other.opt_stat)
    }
}