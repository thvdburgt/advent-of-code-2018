mod datetime;
mod record;

use std::collections::HashMap;

use crate::record::*;

fn create_sleep_record(input: &str) -> HashMap<u32, [u32; 64]> {
    let mut records: Vec<Record> = input.lines().map(|line| line.parse().unwrap()).collect();
    records.sort();

    // create record of minutes asleep for each guard
    let mut hm: HashMap<u32, [u32; 64]> = HashMap::new();
    let mut cur_guard = None;
    let mut sleep_start = None;
    for r in records {
        match r.record_type {
            RecordType::ShiftStart(id) => {
                assert!(sleep_start == None);
                cur_guard = Some(id);
            }
            RecordType::FallsAsleep => {
                assert!(cur_guard != None);
                assert!(sleep_start == None);
                sleep_start = Some(r.datetime.time.minute);
            }
            RecordType::WakesUp => {
                assert!(cur_guard != None);
                assert!(sleep_start != None);
                let guard = cur_guard.unwrap();
                let start = sleep_start.unwrap();
                let end = r.datetime.time.minute;
                let sleep_record = hm.entry(guard).or_insert([0; 64]);
                for m in start..end {
                    sleep_record[m as usize] += 1;
                }
                sleep_start = None;
            }
        }
    }

    hm
}

pub fn solve_puzzle_part_1(input: &str) -> u32 {
    let sleep_record = create_sleep_record(input);

    // Strategy 1:
    // Find the guard that has the most minutes asleep. What minute
    // does that guard spend asleep the most?

    let (id, min_most_asleep, _) = sleep_record
        .iter()
        .fold(
            None,
            |mut acc: Option<(u32, usize, u32)>, (guard, record)| {
                // (id, min most asleep, total minutes asleep)
                let total_minutes_asleep: u32 = record.iter().sum();
                if acc == None || total_minutes_asleep > acc.unwrap().2 {
                    let (m, _) = record
                        .iter()
                        .enumerate()
                        .fold(None, |mut acc: Option<(usize, u32)>, (min, asleep)| {
                            // (min, times asleep)
                            if acc == None || *asleep > acc.unwrap().1 {
                                acc = Some((min, *asleep));
                            }
                            acc
                        })
                        .unwrap();
                    acc = Some((*guard, m, total_minutes_asleep));
                }
                acc
            },
        )
        .unwrap();

    id * min_most_asleep as u32
}

pub fn solve_puzzle_part_2(input: &str) -> u32 {
    let sleep_record = create_sleep_record(input);

    // Strategy 2:
    // Of all guards, which guard is most frequently asleep on the
    // same minute?

    let (id, min_most_asleep, _) = sleep_record
        .iter()
        .fold(
            None,
            |mut acc: Option<(u32, usize, u32)>, (guard, record)| {
                // acc = (id, min most asleep, times asleep that minute)
                let (min_most_asleep, max_times_asleep): (usize, u32) = record
                    .iter()
                    .enumerate()
                    .fold(None, |mut acc: Option<(usize, u32)>, (m, times_asleep)| {
                        // acc = (minute, max_times_asleep)
                        if acc == None || *times_asleep > acc.unwrap().1 {
                            acc = Some((m, *times_asleep));
                        }
                        acc
                    })
                    .unwrap();

                if acc == None || max_times_asleep > acc.unwrap().2 {
                    acc = Some((*guard, min_most_asleep, max_times_asleep));
                }

                acc
            },
        )
        .unwrap();

    id * min_most_asleep as u32
}
