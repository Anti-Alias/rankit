import { useLocation, Location, useNavigate } from "react-router-dom";
import Countdown, { CountdownRenderProps, CountdownRendererFn } from "react-countdown";
import styles from "./Verification.module.css";
import { useState } from "react";
import DigitInput from "../components/DigitInput";
import Button from "../components/Button";
import { useApiClient } from "../components/AppDataProvider";
import { ApiError } from "../utils/apiClient";
import { ERROR_500 } from "../utils/constants";

const DIGIT_LENGTH: number = 6;
type VerificationState = {
    accountId: number,
    accountEmail: string
};

function Verification() {
    
    const [code, setCode]                   = useState("");
    const [error, setError]                 = useState("");
    const [verifiedTime, setVerifiedTime]   = useState<number | null>(null);
    const location                          = useLocation() as Location<VerificationState | null>;
    const navigate                          = useNavigate();
    const client                            = useApiClient();
    const state = location.state;

    if(!state) {
        return <h1>Unexpected Error Occurred</h1>;
    }

    const handleVerification = async () => {
        const shortCode = code.replaceAll(" ", "");
        if(shortCode.length === 0) {
            setError("Required")
        }
        else if(shortCode.length < DIGIT_LENGTH) {
            setError("All values must be filled");
        }
        else {
            try {
                await client.verifyAccount(state.accountId, code);
                setVerifiedTime(Date.now() + 10000)
            }
            catch(e) {
                if(e instanceof ApiError) {
                    setError(e.message);
                }
                else {
                    console.log(e);
                    setError(ERROR_500);
                }
            }
        }
    }

    const countdownRenderer = (props: CountdownRenderProps) => {
        return <span>{props.seconds}</span>
    }
    const onComplete = () => navigate("/login");

    if(!verifiedTime) return (
        <div className={styles.Unverified}>
            <h1>Verify Your Email Address</h1>
            <p>An email was sent to <b>{state.accountEmail}</b> with a verification code.</p>
            <DigitInput value={code} onChange={setCode} length={DIGIT_LENGTH}/>
            <p className={styles.error}>{error}</p>
            <Button onClick={handleVerification}>Verify</Button>
        </div>
    )
    else return (
        <div className={styles.Verified}>
            <h1>Account Verified</h1>
            <p>
                You will be redirected in&nbsp;
                <Countdown
                    date={verifiedTime}
                    renderer={countdownRenderer}
                    onComplete={onComplete}
                />
                &nbsp;seconds.</p>
        </div>
    );
}

export default Verification;
export { type VerificationState };