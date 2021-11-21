use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, LineWriter};
use crate::task::Task;

pub fn read_input(filename: &str) -> Result<Vec<Task>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    let mut lines = reader.lines();
    let n: u32 = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    for line in lines {
        line.map(|l| {
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
