const passwordLengthMin: number = 8;
const passwordSpecialCharacters: string[] = [ '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_' , '=', '+' ];
/// Source: https://emailregex.com/
const emailRegex: RegExp = /^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$/;

export function validateEmail(email: string): string | null {
  if(email.length === 0) {
    return 'Required';
  }
  if(!email.match(emailRegex)) {
    return 'Invalid Email';
  }
  return null;
}

export function validatePassword(password: string): string | null {
  if(password.length === 0) {
    return 'Required';
  }
  if(password.length < passwordLengthMin) {
    return `Password must be at least ${passwordLengthMin} characters`;
  }
  const includesSpecialCharacter = passwordSpecialCharacters.some(char => password.includes(char));
  if(!includesSpecialCharacter) {
    return 'Password must include at least one special character';
  }
  return null;
}

export function validatePasswordVerify(passwordVerify: string, password: string): string | null {
  if(passwordVerify.length === 0) {
    return 'Required';
  }
  if(passwordVerify !== password) {
    return 'Passwords do not match';
  }
  return null;
}
