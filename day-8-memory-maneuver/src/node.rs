use std::collections::HashMap;

pub struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<usize>,
}

impl Node {
    pub fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let mut child_values: HashMap<usize, usize> = HashMap::new();
            let mut value = 0;

            for &n in &self.metadata {
                value += *child_values.entry(n).or_insert_with(|| {
                    if let Some(child) = self.children.get(n - 1) {
                        child.value()
                    } else {
                        0
                    }
                });
            }

            value
        }
    }
}

pub fn parse_nodes(s: &str) -> (Node, usize) {
    let numbers: Vec<usize> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut numbers = numbers.as_slice();

    enum ToRead {
        EndOfNode((usize, usize)),
        StartOfNode,
    }
    let mut to_read: Vec<ToRead> = vec![ToRead::StartOfNode];

    let mut metadata_sum = 0;

    let mut nodes: Vec<Node> = Vec::new();
    while let Some(todo) = to_read.pop() {
        match todo {
            ToRead::EndOfNode((child_quantity, metadata_quantity)) => {
                assert!(nodes.len() >= child_quantity);
                assert!(numbers.len() >= metadata_quantity);

                let metadata = numbers[0..metadata_quantity].to_vec();
                numbers = &numbers[metadata_quantity..];

                metadata_sum += metadata.iter().sum::<usize>();

                let children = {
                    let nodes_len = nodes.len();
                    nodes
                        .drain((nodes_len - child_quantity)..)
                        .map(Box::new)
                        .collect()
                };

                nodes.push(Node { children, metadata });
            }
            ToRead::StartOfNode => {
                assert!(numbers.len() >= 2);

                let child_quantity = numbers[0];
                let metadata_quantity = numbers[1];
                assert!(metadata_quantity > 0);
                numbers = &numbers[2..];

                to_read.push(ToRead::EndOfNode((child_quantity, metadata_quantity)));

                for _ in 0..child_quantity {
                    to_read.push(ToRead::StartOfNode);
                }
            }
        }
    }

    assert_eq!(nodes.len(), 1);
    (nodes.remove(0), metadata_sum)
}
