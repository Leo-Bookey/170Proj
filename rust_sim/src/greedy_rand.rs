use std::collections::BinaryHeap;
use crate::soln::generate_random_schedule;
use crate::task::Task;

pub fn greedy_rand(jobs: &Vec<Task>, num_samples: u64, top_n: u32) -> Vec<Task> {
    let prefixes: Vec<Vec<usize>> = Vec::new();
    let n = jobs.len();
    Vec::new()
}

fn prefix_search(jobs: &Vec<Task>, prefix: &Vec<Task>) -> Vec<Task> {
    Vec::new()
}

fn decision_function(jobs: &Vec<Task>) -> f64 {
    1.0
}

fn get_initial_best(jobs: &mut Vec<Task>, num_samples: u64, top_n: u32) -> Vec<usize> {
    let n = jobs.len();
    let mut best_prefixes: Vec<usize> = Vec::new();

    for i in 0..n {
        let mut jobs_copy = jobs.clone();
        let mut schedule: Vec<Task> = Vec::new();
        schedule.push(jobs.swap_remove(i));
    }

    best_prefixes
}