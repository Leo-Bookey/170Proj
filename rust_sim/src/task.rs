use std::cmp;
use std::f64;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
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