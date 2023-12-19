import { ChangeEvent, FormEvent } from "react";
import styles from "./LogIn.module.css";
import Form from "../components/Form";
import Input from "../components/Input";
import Button from "../components/Button";
import { Link } from "react-router-dom";
import { FormValidator } from "../utils/form";

function validateRequired(inputValue: string): string | void {
    if(!inputValue) {
        return "Required";
    }
}

const Validator = new FormValidator()
    .addValidator("email", "emailError", validateRequired)
    .addValidator("password", "passwordError", validateRequired);

function LogIn() {

    const onFormSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const form = event.currentTarget;
        if(!Validator.validateForm(form)) {
            return
        }
    }

    const onEmailChange = (event: ChangeEvent<HTMLInputElement>) => {
        Validator.validateInput("email", event.currentTarget.value);
    }

    const onPasswordChange = (event: ChangeEvent<HTMLInputElement>) => {
        Validator.validateInput("password", event.currentTarget.value);
    } 

    return (
        <Form onSubmit={onFormSubmit} noValidate>
            <h1 className={styles.header}>Log In</h1>
            <Input
                errorId="emailError"
                type="email"
                name="email"
                displayName="Email"
                onChange={onEmailChange}
            />
            <Input
                errorId="passwordError"
                type="password"
                name="password"
                displayName="Password"
                onChange={onPasswordChange}
            />
            <p className={styles.memberText}>Not a member? <Link to="/signup">Sign Up</Link></p>
            <Button type="submit">Submit</Button>
        </Form>
        
    );

};

export default LogIn;