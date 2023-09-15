import Drawflow from 'drawflow';
import { EditorNodeClassName } from './EditorNodeClassName';

export class EditorNode {

    private html: string = "";
    private data: any = {};
    private inputNbr: number = 0;
    private outputNbr: number = 0;
    private x: number = 0;
    private y: number = 0;

    private constructor(private drawFlow: Drawflow, private className: EditorNodeClassName) { }

    public static create(drawFlow: Drawflow, className: EditorNodeClassName = "ANY") {
        return new EditorNode(drawFlow, className);
    }

    public withName(name: string) {
        this.html = `<div>${name}</div>`;
        return this;
    }

    public withData(data: any) {
        this.data = data;
        return this;
    }

    public withInput() {
        this.html = `
        <div><input type="text" df-name></div>
        `;
        return this;
    }

    public addInput(nbr: number) {
        this.inputNbr = nbr;
        return this;
    }

    public addOutput(nbr: number) {
        this.outputNbr = nbr;
        return this;
    }

    public atCoord(x: number, y: number) {
        this.x = x;
        this.y = y;
        return this;
    }

    public build() {
        this.drawFlow.addNode(
            this.className + '_' + String(Math.round(Math.random() * 10000)),
            this.inputNbr,
            this.outputNbr,
            this.x,
            this.y,
            this.className,
            this.data,
            this.html,
            false
        );
    }
}
