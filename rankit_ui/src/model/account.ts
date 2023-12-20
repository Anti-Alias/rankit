/**
 * Request to make a new account.
*/
export interface CreateRequest {
    name: string,
    email: string,
    password: string,
}

/**
 * Response to a {@link AccountCreateRequest}.
 */
export interface CreateResponse {
    id: number,
    name: string,
    email: string,
}

export type Role = "basic" | "admin" | "root";

export interface Account {
    id: number,
    name: string,
    email: string,
    role: Role
}