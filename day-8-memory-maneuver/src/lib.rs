mod node;

pub fn solve_puzzle_part_1(input: &str) -> usize {
    node::parse_nodes(input).1
}

pub fn solve_puzzle_part_2(input: &str) -> usize {
    node::parse_nodes(input).0.value()
}
