import Drawflow from 'drawflow';
import { EditorNode } from '../EditorNode';
import { ComponentData } from '../model/ComponentData';

export class NewComponentForm {

    public static addToElement(id: HTMLElement, editor: Drawflow) {
        return new NewComponentForm(id, editor);
    }

    private constructor(private id: HTMLElement, private editor: Drawflow) {
        const parent = this.id.parentElement;
        if (parent == null) throw new Error("cannot get parent element");

        const divNewComponent = document.createElement("div");

        // Create a label for "Nom"
        const nameLabel = document.createElement("label");
        nameLabel.textContent = "Nom :";
        const inputComponentName = document.createElement("input");
        inputComponentName.type = "text";
        inputComponentName.id = "inputComponentName";
        nameLabel.appendChild(inputComponentName);

        // Create labels and input fields for "Input number" and "Output number"
        const inputLabel = document.createElement("label");
        inputLabel.setAttribute("for", "Inputnumber");
        inputLabel.textContent = "Input number";
        const inputNumber = document.createElement("input");
        inputNumber.type = "number";
        inputNumber.id = "Inputnumber";
        inputNumber.name = "Inputnumber";
        inputNumber.min = "0";
        inputNumber.max = "10";
        inputNumber.value = "0";
        inputLabel.appendChild(inputNumber);

        const outputLabel = document.createElement("label");
        outputLabel.setAttribute("for", "Outputnumber");
        outputLabel.textContent = "Output number";
        const outputNumber = document.createElement("input");
        outputNumber.type = "number";
        outputNumber.id = "Outputnumber";
        outputNumber.name = "Outputnumber";
        outputNumber.min = "0";
        outputNumber.max = "10";
        outputNumber.value = "0";
        outputLabel.appendChild(outputNumber);

        // Create a label for data
        const dataLabel = document.createElement("label");
        dataLabel.textContent = "data";
        const inputComponentdata = document.createElement("input");
        inputComponentdata.type = "text";
        inputComponentdata.id = "inputComponentdata";
        dataLabel.appendChild(inputComponentdata);

        // Create the "new" button
        const btnNewComponent = document.createElement("button");
        btnNewComponent.id = "btnNewComponent";
        btnNewComponent.textContent = "new";
        btnNewComponent.addEventListener('click', () => this.createNew(inputComponentName, inputNumber, outputNumber, inputComponentdata));

        // Append all elements to the divNewComponent
        divNewComponent.appendChild(nameLabel);
        divNewComponent.appendChild(dataLabel);
        divNewComponent.appendChild(inputLabel);
        divNewComponent.appendChild(outputLabel);
        divNewComponent.appendChild(btnNewComponent);

        // Append divNewComponent to the parent element
        parent.appendChild(divNewComponent);
    }

    private createNew(
        componentName: HTMLInputElement,
        componentInput: HTMLInputElement,
        componentOutput: HTMLInputElement,
        componentData: HTMLInputElement
    ) {
        const name = componentName?.value ?? "test";
        const inputNbr = parseInt(componentInput?.value) ?? 0;
        const outputNbr = parseInt(componentOutput?.value) ?? 0;
        const data = componentData?.value ?? {};
        let parsedData: Partial<ComponentData>;
        try {
            parsedData = JSON.parse(data);
        } catch (error) {
            console.error("data is not JSON : ", data);
            parsedData = JSON.parse('{}');
        }

        EditorNode
            .create(this.editor)
            .withName(name)
            .addInput(inputNbr)
            .addOutput(outputNbr)
            .withData(parsedData)
            .build();
        componentName.value = "";
    }
}
