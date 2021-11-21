use crate::task::Task;
use std::collections::HashSet;
use rand::Rng;
use std::cmp;
use rulinalg::utils::argmax;

const TIMEOUT: u64 = 1440;

pub fn compute_cost(used_jobs: &Vec<Task>) -> f64 {
    let mut cost: f64 = 0.0;
    let mut time: u64 = 0;
    for job in used_jobs {
        time += job.get_duration();
        cost += job.get_late_penalty(time as i64 - job.get_deadline() as i64);
    }
    cost
}

fn is_valid(curr_time: u64, job: Task) -> bool {
    curr_time + job.get_duration() <= TIMEOUT
}

fn remove_random(jobs: &mut Vec<Task>) -> Option<Task> {
    let mut rng = rand::thread_rng();
    if jobs.len() == 0 {
        None
    } else {
        let index = rng.gen_range(0..jobs.len());
        Some(jobs.swap_remove(index))
    }
}

pub fn generate_random_schedule(jobs: &Vec<Task>) -> Vec<Task> {
    let mut jobs_copy = jobs.clone();
    let mut schedule: Vec<Task> = Vec::new();
    let mut curr_time: u64 = 0;
    let n = jobs.len() as u64;
    for i in 0..n {
        let job: Task = remove_random(&mut jobs_copy).unwrap();
        if is_valid(curr_time, job) {
            schedule.push(job);
            curr_time += job.get_duration();
        }
    }
    schedule
}

pub fn sample(jobs: Vec<Task>, num_samples: u64) -> (Vec<Task>, f64) {
    let mut samples: Vec<Vec<Task>> = Vec::new();
    let mut cost: Vec<f64> = Vec::new();
    for _ in 0..num_samples {
        let schedule = generate_random_schedule(&jobs);
        cost.push(compute_cost(&schedule));
        samples.push(schedule);
    }
    let best_idx = argmax(&cost).0;
    (samples[best_idx].clone(), cost[best_idx])
}


