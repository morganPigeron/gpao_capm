use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum DataKey {
    Duration,
}

#[derive(Debug)]
pub struct FlowGraph {
    pub elements: HashMap<String, FlowElement>,
}

#[derive(Debug)]
pub struct FlowElement {
    pub id: i32,
    pub name: String,
    pub data: HashMap<DataKey, String>, // You can use HashMap to represent an empty object in TypeScript
    pub class: String,
    pub html: String,
    pub typenode: bool,
    pub inputs: HashMap<String, FlowElementInput>,
    pub outputs: HashMap<String, FlowElementOutput>,
    pub pos_x: f64, // Use f64 for floating-point numbers
    pub pos_y: f64,
}

impl FlowElement {
    pub fn get_duration(&self) -> usize {
        self.data
            .get(&DataKey::Duration)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct FlowElementInput {
    pub connections: Vec<InConnection>,
}

#[derive(Debug)]
pub struct FlowElementOutput {
    pub connections: Vec<OutConnection>,
}

#[derive(Debug)]
pub struct InConnection {
    pub node: String,
    pub input: String,
}

#[derive(Debug)]
pub struct OutConnection {
    pub node: String,
    pub output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_flow_element() {
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

        let input = FlowElementInput {
            connections: input_connections,
        };

        let output = FlowElementOutput {
            connections: output_connections,
        };

        let element = FlowElement {
            id: 1,
            name: String::from("Element 1"),
            data: HashMap::new(),
            class: String::from("Class 1"),
            html: String::from("<div></div>"),
            typenode: true,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            pos_x: 10.0,
            pos_y: 20.0,
        };

        // Now, you can write assertions to test the behavior of the FlowElement struct
        assert_eq!(element.id, 1);
        assert_eq!(element.name, "Element 1");
        assert_eq!(element.data.len(), 0);
        assert_eq!(element.class, "Class 1");
        assert_eq!(element.html, "<div></div>");
        assert_eq!(element.typenode, true);
        assert_eq!(element.inputs.len(), 0);
        assert_eq!(element.outputs.len(), 0);
        assert_eq!(element.pos_x, 10.0);
        assert_eq!(element.pos_y, 20.0);
    }

    #[test]
    fn test_get_duration() {
        let mut data = HashMap::new();
        data.insert(DataKey::Duration, "42".to_string());

        let element = FlowElement {
            id: 1,
            name: String::from("Element 1"),
            data,
            class: String::from("Class 1"),
            html: String::from("<div></div>"),
            typenode: true,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            pos_x: 10.0,
            pos_y: 20.0,
        };

        // Test when the key is found and parsing is successful
        assert_eq!(element.get_duration(), 42);

        // Test when the key is not found
        let element_no_key = FlowElement {
            id: 1,
            name: String::from("Element 1"),
            data: HashMap::new(), // Empty data map
            class: String::from("Class 1"),
            html: String::from("<div></div>"),
            typenode: true,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            pos_x: 10.0,
            pos_y: 20.0,
        };
        assert_eq!(element_no_key.get_duration(), 0);

        // Test when parsing fails
        let mut data_invalid = HashMap::new();
        data_invalid.insert(DataKey::Duration, "invalid".to_string());
        let element_invalid = FlowElement {
            id: 1,
            name: String::from("Element 1"),
            data: data_invalid,
            class: String::from("Class 1"),
            html: String::from("<div></div>"),
            typenode: true,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            pos_x: 10.0,
            pos_y: 20.0,
        };
        assert_eq!(element_invalid.get_duration(), 0);
    }
}
