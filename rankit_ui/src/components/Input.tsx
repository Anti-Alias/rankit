import { ChangeEventHandler } from 'react';
import styles from './Input.module.css';

interface InputProps {
    id?: string,
    errorId?: string,
    name?: string,
    required?: boolean,
    displayName?: string,
    type?: "text" | "email" | "password" | "number",
    maxLength?: number,
    onChange?: ChangeEventHandler<HTMLInputElement>;
}

function Input(props: InputProps) {
    const { id, errorId, name, required, displayName="", type, maxLength, onChange } = props;

    let displayElem: JSX.Element;
    if(displayName && !required) {
        displayElem = <>{displayName}</>;
    }
    else if(displayName && required) {
        displayElem = <><span className={styles.required}>*</span>{displayName}</>;
    }
    else {
        displayElem = <></>;
    }

    return (<>
        <div>{displayElem}</div>
        <input
            id={id}
            name={name}
            type={type}
            maxLength={maxLength}
            onChange={onChange}
            className={styles.inputClass}
            required={required}
        />
        <p id={errorId} className={styles.error}/>
    </>);
}

export default Input;