mod instruction;
mod step;
mod worker;

use crate::instruction::Instruction;
use crate::step::Step;
use crate::worker::Worker;

use std::collections::{BinaryHeap, HashMap, HashSet};

fn read_instructions(instructions: &str) -> (BinaryHeap<Step>, HashMap<Step, HashSet<char>>) {
    // parse string to instructions
    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut nodes: HashMap<Step, HashSet<char>> = HashMap::new();
    let mut reqs = HashSet::new();

    // create mapping of steps to its requirements
    for instruction in instructions {
        nodes.entry(instruction.step).or_default().insert(instruction.requirement);
        reqs.insert(Step(instruction.requirement));
    }

    // create heap of steps without requirements
    let heap = reqs
        .into_iter()
        .filter(|step| !nodes.contains_key(step))
        .collect();

    // return both
    (heap, nodes)
}

pub fn solve_puzzle_part_1(input: &str) -> String {
    // read instructions to mapping of steps to their requirements and
    // a heap of steps we are able to take
    let (mut heap, nodes) = read_instructions(input);

    // do all steps in correct order
    let mut done: Vec<char> = Vec::new();
    while let Some(Step(step)) = heap.pop() {
        // we have finished step
        done.push(step);
        // add all steps to the heap whose requirement are now met
        nodes.iter()
            .filter(|(_, reqs)| reqs.contains(&step))
            .filter(|(_, reqs)| reqs.iter().all(|req| done.contains(req)))
            .for_each(|(&s, _)| heap.push(s));
    }

    done.iter().collect()
}

fn assemble_sleigh(input: &str, base_duration: usize, nr_of_elves: usize) -> usize {
    // read instructions to mapping of steps to their requirements and
    // a heap of steps we are able to take
    let (mut heap, nodes) = read_instructions(input);

    // distribute the jobs over the workers in correct order
    let mut seconds = 0;
    let mut workers: Vec<_> = (1..=nr_of_elves).map(Worker::new).collect();
    let mut done: Vec<char> = Vec::new();
    loop {
        // distribute jobs to workers
        for worker in workers.iter_mut().filter(|w| w.step.is_none()) {
            if let Some(step) = heap.pop() {
                worker.give_job(step, base_duration);
            }
        }

        // make the workers work and get the result if they're done
        for worker in &mut workers {
            if let Some(Step(step)) = worker.tick() {
                // we have finished step
                done.push(step);
                // add all steps to the heap whose requirement are now met
                nodes.iter()
                    .filter(|(_, reqs)| reqs.contains(&step))
                    .filter(|(_, reqs)| reqs.iter().all(|req| done.contains(req)))
                    .for_each(|(&s, _)| heap.push(s));
            }
        }

        seconds += 1;

        // check if we are done
        if workers.iter().all(|w| w.step.is_none()) && heap.is_empty() {
            break;
        }
    }

    seconds
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    assemble_sleigh(input, 60, 5)
}

#[test]
fn part_2_example() {
    const SAMPLE_INPUT: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    assert_eq!(assemble_sleigh(SAMPLE_INPUT, 0, 2), 15);
}
