use crate::graph::*;
use std::collections::HashMap;
mod graph;

fn main() {
    // Example usage
    let mut elements = HashMap::new();
    let input_connections = vec![
        InConnection {
            node: String::from("node1"),
            input: String::from("input1"),
        },
        // Add more connections as needed
    ];
    let output_connections = vec![
        OutConnection {
            node: String::from("node2"),
            output: String::from("output1"),
        },
        // Add more connections as needed
    ];

    let input = graph::FlowElementInput {
        connections: input_connections,
    };

    let output = graph::FlowElementOutput {
        connections: output_connections,
    };

    let element = graph::FlowElement {
        id: 1,
        name: String::from("Element 1"),
        data: HashMap::new(), // Initialize with an empty HashMap
        class: String::from("Class 1"),
        html: String::from("<div></div>"),
        typenode: true,
        inputs: HashMap::new(),  // Initialize with an empty HashMap
        outputs: HashMap::new(), // Initialize with an empty HashMap
        pos_x: 10.0,             // Use f64 for floating-point numbers
        pos_y: 20.0,
    };

    elements.insert(String::from("element1"), element);

    let graph = graph::FlowGraph { elements };

    // Print the FlowGraph
    println!("{:#?}", graph);
}
