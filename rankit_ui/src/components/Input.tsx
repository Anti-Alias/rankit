import { ChangeEventHandler } from 'react';
import styles from './Input.module.css';

interface InputProps {
    id?: string,
    name?: string,
    type?: "text" | "email" | "password" | "number",
    maxLength?: number,
    onChange?: ChangeEventHandler<HTMLInputElement>;
}

function Input(props: InputProps) {
    return (
        <input
            id={props.id}
            name={props.name}
            type={props.type}
            maxLength={props.maxLength}
            onChange={props.onChange}
            className={styles.inputClass}
        ></input>
    );
}

export default Input;