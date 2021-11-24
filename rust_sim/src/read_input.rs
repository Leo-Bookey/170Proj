use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, LineWriter};
use std::iter::Take;
use crate::task::Task;
use crate::soln::compute_cost;

pub fn read_input(filename: &str) -> Result<Vec<Task>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    let mut lines = reader.lines();
    let _: u32 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    for line in lines {
        let _ = line.map(|l| {
            let mut line = l.split_whitespace();
            let task_id = line.next().unwrap().parse::<u32>().unwrap();
            let task_deadline = line.next().unwrap().parse::<u64>().unwrap();
            let task_duration = line.next().unwrap().parse::<u64>().unwrap();
            let task_reward = line.next().unwrap().parse::<f64>().unwrap();
            input.push(Task::new(task_id, task_deadline, task_duration, task_reward));
        });
    }
    Ok(input)
}

pub fn write_output(filename: &str, output: Vec<Task>) -> Result<(), Error> {
    let file = File::create(filename)?;
    let mut file = LineWriter::new(file);
    // Write each u32 to a line in the file
    for task in output {
        file.write_fmt(format_args!("{}\n", task.get_id()))?;
    }
    Ok(())
}

pub fn read_output(filename: &str) -> Result<Vec<u32>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();
    let mut lines = reader.lines();
    for line in lines {
        let _ = line.map(|l| {
            let task_id = l.parse::<u32>().unwrap();
            output.push(task_id);
        });
    }
    Ok(output)
}

pub fn compare_to_saved(filename_prev: &str, filename_input: &str, new_schedule: Vec<Task>, cost: f64) -> Result<(), Error> {
    let prev = read_output(filename_prev)?;
    let input = read_input(filename_input)?;
    let schedule = prev.iter().map(|idx| input.get((idx-1) as usize).unwrap()).cloned().collect::<Vec<Task>>();
    let old_cost = compute_cost(&schedule);
    if old_cost <= cost {
        write_output(filename_prev, new_schedule).unwrap();
    }
    Ok(())   
}