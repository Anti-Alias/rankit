import { ChangeEvent, FormEvent, useState } from "react";
import { jwtDecode } from "jwt-decode";
import styles from "./LogIn.module.css";
import Form from "../components/Form";
import Input from "../components/Input";
import Button from "../components/Button";
import { Link, useNavigate } from "react-router-dom";
import { FormValidator } from "../utils/form";
import { useApiClient } from "../components/AppDataProvider";
import { ApiError } from "../utils/apiClient";
import { ERROR_500 } from "../utils/constants";
import { useAccountState as useAccount } from "../components/AccountProvider";
import { Account } from "../model/account";

function validateRequired(inputValue: string): string | void {
    if(!inputValue) {
        return "Required";
    }
}

const Validator = new FormValidator()
    .addValidator("email", "emailError", validateRequired)
    .addValidator("password", "passwordError", validateRequired);

function LogIn() {

    const [error, setError] = useState("");
    const client = useApiClient();
    const { setAccount } = useAccount();
    const navigate = useNavigate();

    const onFormSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const form = event.currentTarget;
        if(!Validator.validateForm(form)) {
            return
        }
        const emailInput = form.elements.namedItem("email") as HTMLInputElement;
        const passwordInput = form.elements.namedItem("password") as HTMLInputElement;
        try {
            const jwt = await client.loginAccount(emailInput.value, passwordInput.value);
            const account = jwtDecode(jwt) as Account;
            setAccount(account);
            navigate("/");
        }
        catch(e) {
            if(e instanceof ApiError) {
                setError(e.message);
            }
            else {
                setError(ERROR_500);
                console.error(e);
            }
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
            <p className={styles.error}>{error}</p>
        </Form>
        
    );

};

export default LogIn;