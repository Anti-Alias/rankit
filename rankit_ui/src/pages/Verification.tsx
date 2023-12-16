import { useLocation, Location } from "react-router-dom";

type VerificationState = { email: string };

function Verification() {
    const location = useLocation() as Location<VerificationState>;
    return <>
        <h1>Verify your email address</h1>
        <p>
            An email was sent to <b>{location.state.email}</b>.
            Please click on the link in the email sent to complete the signup process.
        </p>
    </>
}

export default Verification;
export { type VerificationState };