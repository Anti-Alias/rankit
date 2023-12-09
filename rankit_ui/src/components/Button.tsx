import { ReactElement } from 'react';
import styles from './Button.module.css';

interface Props {
    theme?: "primary" | "alternate",
    type?: "button" | "submit" | "reset" | undefined,
    children: string | ReactElement,
}

const Button = ({theme = "primary", type = "button", children}: Props) => {
    const themeName = styles[theme];
    const buttonName = styles.Button;
    return <button type={type} className={`${buttonName} ${themeName}`}>
        {children}
    </button>
};


export default Button;