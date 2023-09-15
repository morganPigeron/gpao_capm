import Drawflow from 'drawflow';
import { FlowGraph } from '../FlowGraph';

export class ExportButton {

    private constructor(private id: HTMLElement, private editor: Drawflow) {

        const parent = this.id.parentElement;
        if (parent == null) throw new Error("cannot get parent element");

        const btnExport = document.createElement("button");
        btnExport.textContent = "Export";
        parent.appendChild(btnExport);
        btnExport.addEventListener('click', () => this.export());
    }

    public static addToElement(id: HTMLElement, editor: Drawflow) {
        return new ExportButton(id, editor);
    }

    private export() {
        const data: FlowGraph = this.editor.export().drawflow.Home.data as unknown as FlowGraph;
        console.log(data);
        for (const key in data) {
            console.log(key);
            console.log(JSON.stringify(data[key]));
        }
    }
}
