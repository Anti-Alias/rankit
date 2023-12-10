import { MouseEventHandler, ReactElement } from 'react';
import styles from './Button.module.css';

interface Props {
    color?: "primary" | "secondary",
    type?: "button" | "submit" | "reset",
    size?: "normal" | "small",
    children: string | ReactElement,
    onClick?: MouseEventHandler,
}

function Button(props: Props) {
    const {
        color = "primary",
        type = "button",
        size = "normal",
        onClick,
        children
    } = props;
    const buttonName = styles.Button;
    const themeName = styles[color];
    const sizeName = styles[size];
    return (
        <button
            type={type}
            className={`${buttonName} ${themeName} ${sizeName}`}
            onClick={onClick}
        >
        {children}
        </button>
    )
};


export default Button;