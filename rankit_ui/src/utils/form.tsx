type Validator = (input: string) => string | void | null;

type ValidatorData = {
    messageContainerId: string,
    validator: Validator
}

class FormValidator {

    private validators: Map<string, ValidatorData> = new Map();

    addValidator(inputName: string, messageContainerId: string, validator: Validator): FormValidator {
        const data = { messageContainerId, validator };
        this.validators.set(inputName, data);
        return this;
    }

    /**
     * Validates a form.
     * Returns true if all validators were successful.
     */
    validate(form: HTMLFormElement): boolean {
        let success = true;
        for(const [inputName, validatorData] of this.validators.entries()) {

            // Fetches DOM elements
            const inputElem = form[inputName] as HTMLInputElement | undefined;
            if(!inputElem) {
                throw new Error(`Form input with name ${inputName} not found`);
            }
            const messageElem = document.getElementById(validatorData.messageContainerId);
            if(!messageElem) {
                throw new Error(`Element with id ${validatorData.messageContainerId} not found`);
            }

            // Validates input and sets error message if emitted.
            const errorMessage = validatorData.validator(inputElem.value);
            if(errorMessage) {
                if(!messageElem) {
                    throw new Error(`Element with id ${validatorData.messageContainerId} not found`);
                }
                messageElem.innerHTML = errorMessage;
                success = false;
            }
            else {
                messageElem.innerHTML = "";
            }
        }
        return success;
    }
}

export { FormValidator, type Validator };