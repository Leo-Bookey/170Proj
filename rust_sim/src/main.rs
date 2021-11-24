mod read_input;
mod task;
mod soln;
mod greedy_rand;

use soln::sample;
use read_input::write_output;
use std::time::Instant;
use greedy_rand::greedy_rand;

fn main() {
    let file = "153";
    let jobs = read_input::read_input(format!("./input/{}.in", file).as_str()).unwrap();
    let now = Instant::now();
    let (seq, cost) = sample(jobs, 1000000);
    // let (seq, cost) = greedy_rand(&jobs, 100, 3);
    println!("Time Elapse in ms: {}", now.elapsed().as_millis());
    println!("Best Reward: {:?}", cost);
    println!("Sequence: {:?}", seq.iter().map(|x| x.get_id()).collect::<Vec<u32>>());
    write_output(format!("./output/{}.out", file).as_str(), seq);
    // println!("{:?}", sample_prefix(&jobs, vec![*jobs.get(0).unwrap()], 1000000));

}
