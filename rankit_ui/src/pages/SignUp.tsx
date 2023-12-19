import { ChangeEvent, FormEvent, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { FormValidator } from '../utils/form';
import { useApiClient } from '../components/AppDataProvider';
import styles from './SignUp.module.css';
import Button from '../components/Button';
import Input from '../components/Input';
import Form from '../components/Form';
import validator from 'validator';
import * as account from '../model/account';
import { ApiError } from '../utils/apiClient';
import { VerificationState } from './Verification';
import { ERROR_500 } from '../utils/text';

const MinPasswordLength: number = 8;
const UsernameRegex: RegExp = /^[a-zA-Z0-9_-]{4,32}$/;
const AlphaNumericRegex: RegExp = /[a-zA-Z0-9]{1,32}/;
const SpecialCharacterRegex: RegExp = /[~!@#$%^&*]/;

const Validator = new FormValidator()
    .addValidator("email", "emailError", validateEmail)
    .addValidator("username", "usernameError", validateUsername)
    .addValidator("password", "passwordError", validatePassword);

function parseForm(form: HTMLFormElement): account.CreateRequest {
    const emailInput = form.elements.namedItem("email") as HTMLInputElement;
    const usernameInput = form.elements.namedItem("username") as HTMLInputElement;
    const passwordInput = form.elements.namedItem("password") as HTMLInputElement;
    return {
        "email": emailInput.value,
        "name": usernameInput.value,
        "password": passwordInput.value,
    }
}

function validateEmail(email: string): string | void {
    if(!email) {
        return "Required";
    }
    if(!validator.isEmail(email)) {
        return "Invalid email";
    }
}

function validateUsername(username: string): string | void {
    if(!username) {
        return "Required";
    }
    if(!UsernameRegex.test(username)) {
        return "Invalid username";
    }
}

function validatePassword(password: string): string | void {
    if(!password) {
        return "Required";
    }
    if(password.length < MinPasswordLength) {
        return `Password must be at least ${MinPasswordLength} characters`
    }
    if(!AlphaNumericRegex.test(password)) {
        return "Requires at least one alphanumeric character";
    }
    if(!SpecialCharacterRegex.test(password)) {
        return "Requires at least one special character";
    }
}


function SignUp() {

  const [error, setError] = useState("");
  const navigate = useNavigate();
  const client = useApiClient();

  const onFormSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const form = event.currentTarget;
        if(!Validator.validateForm(form)) {
            return
        }
        try {
            const request = parseForm(form);
            const response = await client.createAccount(request);
            const state: VerificationState = {
                accountId: response.id,
                accountEmail: response.email,
            }
            navigate("/verification", { state });
        }
        catch(e) {
            if(e instanceof ApiError) {
                setError(e.message);
            }
            else {
                setError(ERROR_500);
            }
        }
    }

    const onEmailChange = (event: ChangeEvent<HTMLInputElement>) => {
        Validator.validateInput("email", event.currentTarget.value);
    }

    const onUsernameChange = (event: ChangeEvent<HTMLInputElement>) => {
        Validator.validateInput("username", event.currentTarget.value);
    }

    const onPasswordChange = (event: ChangeEvent<HTMLInputElement>) => {
        Validator.validateInput("password", event.currentTarget.value);
    } 

    return (
        <Form onSubmit={onFormSubmit} noValidate>
            <h1 className={styles.header}>Sign Up</h1>
            <Input
                required
                errorId="emailError"
                type="email"
                name="email"
                displayName="Email"
                onChange={onEmailChange}
            />
            <Input
                required
                errorId="usernameError"
                type="text"
                name="username"
                displayName="Username"
                onChange={onUsernameChange}
            />
            <Input
                required
                errorId="passwordError"
                type="password"
                name="password"
                displayName="Password"
                onChange={onPasswordChange}
            />
            <p className={styles.memberText}>Already a member? <Link to="/signup">Log In</Link></p>
            <Button type="submit">Submit</Button>
            <p className={styles.unexpectedError}>{error}</p>    
        </Form>
    );
};


export default SignUp;