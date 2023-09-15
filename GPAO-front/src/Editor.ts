import Drawflow from 'drawflow';
import { ExportButton } from './components/ExportButton';
import { NewComponentForm } from './components/NewComponentForm';

export class Editor {

    private editor: Drawflow;

    constructor() {
        const id = document.getElementById("drawflow")!;
        this.editor = new Drawflow(id);
        this.editor.start();
        NewComponentForm.addToElement(id, this.editor);
        ExportButton.addToElement(id, this.editor);
    }

    public getRef() {
        return this.editor;
    }
}