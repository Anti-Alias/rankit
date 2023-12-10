import { FormEvent, useState } from 'react';
import Button from '../components/Button';
import styles from './SignUp.module.css';
import { Link } from 'react-router-dom';
import { FormValidator } from '../utils/form';
import validator from 'validator';


const MinPasswordLength: number = 8;
const UsernameRegex: RegExp = /^[a-zA-Z0-9_-]{4,32}$/;
const AlphaNumericRegex: RegExp = /[a-zA-Z0-9]{8,32}/;
const SpecialCharacterRegex: RegExp = /[~!@#$%^&*]/;

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

const Validator = new FormValidator()
  .addValidator("email", "emailError", validateEmail)
  .addValidator("username", "usernameError", validateUsername)
  .addValidator("password", "passwordError", validatePassword);

function SignUp() {

  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const success = Validator.validate(event.currentTarget);
    if(success) {
      console.log("Valid form!");
    }
  }

  return (
    <form className={styles.SignUp} onSubmit={submit} noValidate>
      <h1>Sign Up</h1>
      <label className={styles.inputWrapper}>
        <span className={styles.label}>Email</span>
        <input type="email" name="email"/>
        <p id="emailError" className={styles.error}/>
      </label>
      <label className={styles.inputWrapper}>
        <span className={styles.label}>Username</span>
        <input type="text" name="username"/>
        <p id="usernameError" className={styles.error}/>
      </label>
      <label className={styles.inputWrapper}>
        <span className={styles.label}>Password</span>
        <input type="password" name="password"/>
        <p id="passwordError" className={styles.error}/>
      </label>
      <p className={styles.memberText}>Already a member? <Link to="/login">Log In</Link></p>
      <Button type="submit">Submit</Button>
    </form>
  );
};

export default SignUp;