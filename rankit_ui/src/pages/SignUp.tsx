import { ChangeEvent, FormEvent, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { FormValidator } from '../utils/form';
import { useApiClient } from '../components/AppDataProvider';
import styles from './SignUp.module.css';
import Button from '../components/Button';
import validator from 'validator';
import * as account from '../model/account';

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

  const [isError, setIsError] = useState<boolean>(false);
  const navigate = useNavigate();
  const client = useApiClient();

  const onFormSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const form = event.currentTarget;
        if(!Validator.validateForm(form)) {
            return
        }
        try {
            const createAccountRequest = parseForm(form);
            await client.createAccount(createAccountRequest);
            navigate("/verification", { state: { "email": createAccountRequest.email }});
        }
        catch {
            setIsError(true);
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
        <form className={styles.SignUp} onSubmit={onFormSubmit} noValidate>
            <h1>Sign Up</h1>
            <label className={styles.inputWrapper}>
                <span className={styles.label}>Email</span>
                <input type="email" name="email" onChange={onEmailChange}/>
                <p id="emailError" className={styles.error}/>
            </label>
            <label className={styles.inputWrapper}>
                <span className={styles.label}>Username</span>
                <input type="text" name="username" onChange={onUsernameChange}/>
                <p id="usernameError" className={styles.error}/>
            </label>
            <label className={styles.inputWrapper}>
                <span className={styles.label}>Password</span>
                <input type="password" name="password" onChange={onPasswordChange}/>
                <p id="passwordError" className={styles.error}/>
            </label>
            <p className={styles.memberText}>Already a member? <Link to="/login">Log In</Link></p>
            <Button type="submit">Submit</Button>
            {isError && <p className={styles.unexpectedError}>Something went wrong on our end. Please try again.</p>}
        </form>
    );
};


export default SignUp;