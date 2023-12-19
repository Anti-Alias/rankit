import { useState } from "react";
import useDigitInput from "react-digit-input";
import styles from "./DigitInput.module.css";

interface DigitInputProps {
    length?: number,
    value?: string,
    onChange: (value: string) => void,
}

function DigitInput(props: DigitInputProps) {
    const { length = 6, value = "", onChange } = props;
    const digits = useDigitInput({
      acceptedCharacters: /^[0-9]$/,
      length: 6,
      value,
      onChange,
    });

    const elements: Array<JSX.Element> = [];
    for(let i=0; i<length; i++) {
        elements.push(<input key={i} inputMode="decimal" autoFocus {...digits[i]}/>);
    }

    return (
        <div className={styles.DigitInput}>
            {elements}
        </div>
    );
}

export default DigitInput;