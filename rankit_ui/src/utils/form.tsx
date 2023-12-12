type Validator      = (input: string) => string | void | null;
type ValidatorData  = { id: string, validator: Validator };

/**
 * Simplifies the process of validating forms and 
*/
class FormValidator {

    private validators: Map<string, ValidatorData> = new Map();

    /**
     * Adds a validator for a particular form input.
     * @param inputName Name of the form input.
     * @param messageContainerId ID of the input's error container where the error message should be placed.
     * @param validator Function validates the input.
     * @returns void if successful, and a string representing the error message upon failure.
     */
    addValidator(inputName: string, messageContainerId: string, validator: Validator): FormValidator {
        const data = { id: messageContainerId, validator };
        this.validators.set(inputName, data);
        return this;
    }

    /**
     * Validates a native form.
     * @returns true if all validators were successful.
     */
    validate(form: HTMLFormElement): boolean {
        let success = true;
        for(const [inputName, validatorData] of this.validators.entries()) {

            // Fetches input / error container DOM elements using ids.
            const inputElem = form[inputName] as HTMLInputElement | undefined;
            if(!inputElem) {
                throw new Error(`Form input with name ${inputName} not found`);
            }
            const messageElem = document.getElementById(validatorData.id);
            if(!messageElem) {
                throw new Error(`Element with id ${validatorData.id} not found`);
            }

            // Validates input and sets error message if emitted by validator.
            const errorMessage = validatorData.validator(inputElem.value);
            if(errorMessage) {
                if(!messageElem) {
                    throw new Error(`Element with id ${validatorData.id} not found`);
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