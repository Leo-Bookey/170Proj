from time import time
from typing import List
import numpy as np
from parse import read_input_file, read_output_file, write_output_file
from Task import Task
import random

TIME_LIMIT = 1440

def compute_cost(used_jobs: List[Task]):
    '''
    Compute the cost of the solution
    '''
    cost = curr_time = 0
    for job in used_jobs:
        cost += job.get_late_benefit(curr_time + job.get_duration() - job.get_deadline())
        curr_time += job.get_duration()
    return cost

def print_compute_cost(used_jobs: List[Task]):
    '''
    Compute the cost of the solution
    '''
    cost = curr_time = 0
    for job in used_jobs:
        print(cost, curr_time)
        cost += job.get_late_benefit(curr_time - job.get_deadline())
        curr_time += job.get_duration()
    return cost

def _is_valid(curr_time, job: Task):
    '''
    Verify that the deadline of the job ends before TIME_LIMIT after the current time
    '''
    return curr_time + job.get_duration() <= TIME_LIMIT


def gen_random_input(jobs: List[Task], seed=123):
    '''
    Generate a random maximal ordering of jobs
    '''
    n = len(jobs)
    job_set = set(jobs)
    np.random.seed(seed)
    used_jobs = []
    curr_time = 0
    for _ in range(n):
        job = random.sample(job_set, 1)[0]

        if not _is_valid(curr_time, job):
            job_set.remove(job)
            continue

        used_jobs.append(job)
        job_set.remove(job)
        curr_time += job.duration
    return used_jobs
    
def sample(jobs, n):
    '''
    Sample n jobs from the input file
    '''
    schedules = np.vectorize(lambda x: gen_random_input(jobs))(np.ones(n, int))
    costs = np.vectorize(compute_cost)(schedules)
    min_cost_idx = np.argmax(costs)
    return schedules[min_cost_idx], costs[min_cost_idx]

if __name__ == '__main__':
    # n = 1000000
    jobs = read_input_file('153.in')
    # schedule, cost = sample(jobs, n)
    # print("100", cost)
    schedule = read_output_file('153.out')
    print(compute_cost([jobs[s] for s in schedule]))
    # print_compute_cost(schedule)
    # write_output_file('100.out', [j.get_task_id() for j in schedule])
    # # print_compute_cost(schedule)

    # jobs = read_input_file('126.in')
    # print("126", sample(jobs, n)[1])

    # jobs = read_input_file('153.in')
    # print("153", sample(jobs, n)[1])
    # jobs = read_input_file('5.in')
    # schedule, cost = sample(jobs, n)
    # print(cost)
    # write_output_file('100.out', [j.get_task_id() for j in schedule])
