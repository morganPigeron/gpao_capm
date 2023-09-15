export interface FlowGraph {
    [key: string]: FlowElement;
}

export interface FlowElement {
    id: number;
    name: string;
    data: {};
    class: string;
    html: string;
    typenode: boolean;
    inputs: {
        [key: string]: FlowElementInput;
    };
    outputs: {
        [key: string]: FlowElementOutput;
    };
    pos_x: number;
    pos_y: number;
}

export interface FlowElementInput {
    connections: InConnection[];
}

export interface FlowElementOutput {
    connections: OutConnection[];
}

export interface InConnection {
    node: string;
    input: string;
}

export interface OutConnection {
    node: string;
    output: string;
}