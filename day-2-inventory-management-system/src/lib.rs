use std::collections::HashMap;

pub fn solve_puzzle_part_1(input: &str) -> u32 {
    // count all occurences of a 2-char, and 3-char count in a tuple
    let count_tuple = input.split_whitespace().fold((0, 0), |acc, id| {
        // count chars of id in a hash map (char -> count)
        let mut char_count: HashMap<char, u32> = HashMap::new();
        for c in id.chars() {
            *char_count.entry(c).or_default() += 1;
        }

        // go over the counts, if 2 or 3 mark as seen
        // if both have been seen no need in continuing
        let mut seen = (false, false);
        for &count in char_count.values() {
            if count == 2 {
                seen.0 = true;
            } else if count == 3 {
                seen.1 = true;
            }
            if seen == (true, true) {
                break;
            }
        }
        // return the accumulator with 1 added if that count is seen
        (acc.0 + u32::from(seen.0), acc.1 + u32::from(seen.1))
    });

    // multiply the counts to get the checksum
    count_tuple.0 * count_tuple.1
}

pub fn solve_puzzle_part_2(input: &str) -> Option<String> {
    let ids: Vec<_> = input.split_whitespace().collect();

    // compare each pair of id
    for (i, id1) in ids.iter().enumerate() {
        'id_compare: for id2 in ids[i + 1..].iter() {
            // look for a difference found between the ids
            // if more than one are found go to the next pair
            let mut difference_found = false;
            for (c1, c2) in id1.chars().zip(id2.chars()) {
                if c1 != c2 {
                    if difference_found {
                        continue 'id_compare;
                    } else {
                        difference_found = true;
                    }
                }
            }

            if !difference_found {
                // the pair consisted of twice the same id
                continue;
            } else {
                // found two ids with a single difference,
                // create a string by removing the differing character
                let s  = id1
                    .chars()
                    .zip(id2.chars())
                    .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                    .collect();
                return Some(s);
            }
        }
    }

    return None;
}
