mod read_input;
mod task;
mod soln;
mod greedy_rand;

use soln::sample;
use read_input::write_output;

fn main() {
    let jobs = read_input::read_input("./20.in").unwrap();
    let (seq, cost) = sample(jobs, 1000000);
    println!("{:?}", cost);
}
