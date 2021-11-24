#![allow(dead_code)]
use crate::task::{Task, MaxTask};
use crate::soln::{compute_cost, remove_random, is_valid};
use rayon::prelude::*;
use rulinalg::utils::argmax;
use std::collections::HashMap;
use std::iter::Take;
use std::sync::{Arc, Mutex};

// Get the max(n, len(vec)) largest MaxTasks in a vector
fn get_n_largest(vec: &mut Vec<MaxTask>, n: u16) -> Vec<MaxTask> {
    vec.sort_by(|a, b| {
        b.partial_cmp(&a).unwrap()
    });
    vec.truncate(n as usize);
    vec.to_vec()
}

pub fn greedy_rand(jobs: &Vec<Task>, num_samples: u64, top_n: u16) -> (Vec<Task>, f64) {
    let mut prefixes: Vec<Vec<Task>> = Vec::new();
    for i in 0..jobs.len() {
        prefixes.push(vec![jobs.get(i).unwrap().clone()]);
    }
    // let curr_best = Arc::new(Mutex::new(0.0));
    // let best_vec = Arc::new(Mutex::new(Vec::new()));
    let mut curr_best = 0.0;
    let mut best_vec = Vec::new();
    let mut i = 0;
    while prefixes.len() > 0 && i < (jobs.len()-1) as u16 {
        // let new_prefixes:Arc<Mutex<Vec<MaxTask>>> = Arc::new(Mutex::new(Vec::new()));
        // prefixes.clone().par_iter().for_each(|prefix| {
        //     let prefix_max_tasks = prefix_search(jobs, &mut prefix.clone(), num_samples);
        //     let mut new_prefixes_lock = new_prefixes.lock().unwrap();
        //     (*new_prefixes_lock).extend(prefix_max_tasks.clone());
        //     let n_largest = get_n_largest(&mut (*new_prefixes_lock), top_n);
        //     (*new_prefixes_lock).clear();
        //     (*new_prefixes_lock).extend(n_largest);
        //     for max_task in prefix_max_tasks {
        //         let mut curr_best_lock = curr_best.lock().unwrap();
        //         let mut best_vec_lock = best_vec.lock().unwrap();
        //         if max_task.get_max_cost() > *curr_best_lock {
        //             *curr_best_lock += max_task.get_max_cost() - *curr_best_lock;
        //             (*best_vec_lock).clear();
        //             (*best_vec_lock).extend(max_task.get_max_seq().clone());
        //         }
        //     }
        // });
        // prefixes.clear();
        // i += 1;
        // for max_task in new_prefixes.lock().unwrap().iter() {
        //     prefixes.push(max_task.get_max_seq()[0..i as usize].to_vec());
        // }
        let mut new_prefixes: Vec<MaxTask> = Vec::new();
        for prefix in prefixes.clone() {
            let prefix_max_tasks = prefix_search(jobs, &mut prefix.clone(), num_samples);
            new_prefixes.extend(prefix_max_tasks.clone());
            new_prefixes = get_n_largest(&mut new_prefixes, top_n);
            for max_task in prefix_max_tasks {
                if max_task.get_max_cost() > curr_best {
                    curr_best = max_task.get_max_cost();
                    best_vec = max_task.get_max_seq().clone();
                    // println!("{}", i);
                }
            }
        }
        prefixes.clear();
        i += 1;
        for max_task in new_prefixes.iter() {
            prefixes.push(max_task.get_max_seq()[0..i as usize].to_vec());
        }
    }
    // let ret_vec = best_vec.lock().unwrap().clone();
    // let best_cost = *curr_best.lock().unwrap();
    // (ret_vec, best_cost)
    (best_vec, curr_best)
}

pub fn greedy_rand_par(jobs: &Vec<Task>, num_samples: u64, top_n: u16) -> (Vec<Task>, f64) {
    let mut prefixes: Vec<Vec<Task>> = Vec::new();
    for i in 0..jobs.len() {
        prefixes.push(vec![jobs.get(i).unwrap().clone()]);
    }
    let curr_best = Arc::new(Mutex::new(0.0));
    let best_vec = Arc::new(Mutex::new(Vec::new()));
    let mut i = 0;
    while prefixes.len() > 0 && i < (jobs.len()-1) as u16 {
        let new_prefixes:Arc<Mutex<Vec<MaxTask>>> = Arc::new(Mutex::new(Vec::new()));
        prefixes.clone().par_iter().for_each(|prefix| {
            let prefix_max_tasks = prefix_search(jobs, &mut prefix.clone(), num_samples);
            let mut new_prefixes_lock = new_prefixes.lock().unwrap();
            (*new_prefixes_lock).extend(prefix_max_tasks.clone());
            let n_largest = get_n_largest(&mut (*new_prefixes_lock), top_n);
            (*new_prefixes_lock).clear();
            (*new_prefixes_lock).extend(n_largest);
            for max_task in prefix_max_tasks {
                let mut curr_best_lock = curr_best.lock().unwrap();
                let mut best_vec_lock = best_vec.lock().unwrap();
                if max_task.get_max_cost() > *curr_best_lock {
                    *curr_best_lock += max_task.get_max_cost() - *curr_best_lock;
                    (*best_vec_lock).clear();
                    (*best_vec_lock).extend(max_task.get_max_seq().clone());
                }
            }
        });
        prefixes.clear();
        i += 1;
        for max_task in new_prefixes.lock().unwrap().iter() {
            prefixes.push(max_task.get_max_seq()[0..i as usize].to_vec());
        }
    }
    let ret_vec = best_vec.lock().unwrap().clone();
    let best_cost = *curr_best.lock().unwrap();
    (ret_vec, best_cost)
}
// Space complexity can be optimized further by not storing all the MaxTasks, but idt  it'll matter
// so i'll leave it for now.
fn prefix_search(jobs: &Vec<Task>, prefix: &mut Vec<Task>, num_samples: u64) -> Vec<MaxTask> {
    let mut max_tasks: Vec<MaxTask> = Vec::new();
    // let mut jobs_copy = jobs.clone();
    let mut jobs_copy: HashMap<u32, Task> = HashMap::new();
    for job in jobs.iter() {
        jobs_copy.insert(job.get_id(), job.clone());
    }
    let mut start_time = 0;
    for task in prefix.iter() {
        start_time += task.get_duration();
        jobs_copy.remove(&task.get_id());
    }

    for (idx, job) in jobs_copy.iter() {
        if is_valid(start_time, *job) {
            let mut prefix_copy = prefix.clone();
            prefix_copy.push(*job);
            let mut jobs_copy_copy = jobs_copy.clone();
            jobs_copy_copy.remove(idx);
            let curr_max_task = sample_prefix(&jobs_copy_copy, prefix_copy, num_samples);
            max_tasks.push(curr_max_task);
        }
    }
    max_tasks
}

// Returns the best task list and the optimization funciton called on the cost vector
pub fn sample_prefix(jobs: &HashMap<u32,Task>, prefix: Vec<Task>, num_samples: u64) -> MaxTask {
    let mut samples: Vec<Vec<Task>> = Vec::new();
    let mut cost: Vec<f64> = Vec::new();
    let jobs_vec: Vec<Task> = jobs.values().copied().collect();
    for _ in 0..num_samples {
        let schedule = generate_random_schedule_with_prefix(&jobs_vec, &prefix);
        cost.push(compute_cost(&schedule));
        samples.push(schedule);
    }
    let best_idx = argmax(&cost).0;
    MaxTask::new(samples[best_idx].clone(), cost[best_idx], optimizaiton_function(cost))
}

// Randomly generate a valid schedule given a valid prefix of jobs
fn generate_random_schedule_with_prefix(jobs: &Vec<Task>, prefix: &Vec<Task>) -> Vec<Task> {
    let mut jobs_copy = jobs.clone();
    let mut schedule: Vec<Task> = prefix.clone();
    let mut curr_time: u64 = prefix.iter().map(|task| task.get_duration()).sum::<u64>();
    let n = jobs.len() as u64;
    for _ in 0..n {
        let job: Task = remove_random(&mut jobs_copy).unwrap();
        if is_valid(curr_time, job) {
            schedule.push(job);
            curr_time += job.get_duration();
        }
    }
    schedule
}



// Statistics Module
fn optimizaiton_function(cost: Vec<f64>) -> f64 {
    let mean = mean(&cost).unwrap();
    let sd = std_deviation(&cost).unwrap();
    mean + 3.0 * sd
    //argmax(&cost).1
    //sd
}

fn mean(data: &Vec<f64>) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}