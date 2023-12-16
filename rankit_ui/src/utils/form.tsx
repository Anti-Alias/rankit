type Validator = (input: string) => string | void | null;
interface ValidatorData {
    id: string,
    validator: Validator
};

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
    validateForm(form: HTMLFormElement): boolean {
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

    /**
     * Validates a specific form input. Call when the input changes and the user has clicked off of it.
     * @returns true if validator was successful.
     */
    validateInput(inputName: string, inputValue: string): boolean {

        // Fetches validator data / 
        const validatorData = this.validators.get(inputName);
        if(!validatorData) {
            throw new Error(`Validator with name ${inputName} not found`);
        }

        // Fetches input / error container DOM elements using ids.
        const messageElem = document.getElementById(validatorData.id);
        if(!messageElem) {
            throw new Error(`Element with id ${validatorData.id} not found`);
        }

        // Validates input and sets error message if emitted by validator.
        const errorMessage = validatorData.validator(inputValue);
        if(errorMessage) {
            if(!messageElem) {
                throw new Error(`Element with id ${validatorData.id} not found`);
            }
            messageElem.innerHTML = errorMessage;
            return false;
        }
        else {
            messageElem.innerHTML = "";
        }

        return true;
    }
}

export { FormValidator, type Validator };