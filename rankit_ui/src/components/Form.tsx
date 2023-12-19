import { FormEventHandler, ReactNode } from "react";
import styles from "./Form.module.css";

interface FormProps {
    onSubmit?: FormEventHandler<HTMLFormElement>,
    noValidate?: boolean
    children: ReactNode
}

function Form(props: FormProps) {
    const { onSubmit, noValidate, children } = props;
    return (
        <form className={styles.customForm} onSubmit={onSubmit} noValidate={noValidate}>
            {children}
        </form>
    );
}

export default Form;
export { type FormProps }