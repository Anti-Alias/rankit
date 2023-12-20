import * as account from "../model/account";
import { encode } from "base-64";

/**
 * Error that occurred
 */
class ApiError extends Error {
    constructor(public message: string, public responseCode: number) {
        super(message);
    }
}

class ApiClient {

    constructor(private baseUrl: string) {}

    /** 
     * @param request Creates a new, unverified account, sending an email with a unique code.
     * @returns User created with a unique id.
     */
    async createAccount(request: account.CreateRequest): Promise<account.CreateResponse> {
        const response = await this.post("account", request);
        if(!response.ok) {
            throw new ApiError(await response.text(), response.status);
        }
        return response.json();
    }

    /**
     * Verifies an unverified account using an account id and a code.
     */
    async verifyAccount(id: number, code: string): Promise<void> {
        const response = await this.post(`account/${id}/verify/${code}`);
        if(!response.ok) {
            throw new ApiError(await response.text(), response.status);
        }
    }

    /**
     * Logs into an account.
     * @returns JWT string.
     */
    async loginAccount(email: string, password: string): Promise<string> {
        const fullUrl = `${this.baseUrl}/login`;
        const response = await fetch(fullUrl, {
            method: "POST",
            headers: {
                authorization: "Basic " + encode(`${email}:${password}`)
            }
        });
        if(!response.ok) {
            throw new ApiError(await response.text(), response.status);
        }
        return await response.text();
    }

    private async get(url: string): Promise<Response> {
        const fullUrl = `${this.baseUrl}/${url}`;
        const init = {
            method: "GET",
            headers: { "Content-Type": "application/json" },
        };
        return await fetch(fullUrl, init);
    }

    private async post<B>(url: string, body?: B): Promise<Response> {
        const fullUrl = `${this.baseUrl}/${url}`;
        const init: RequestInit = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
        };
        if(body) {
            init.body = JSON.stringify(body);
        }
        return await fetch(fullUrl, init);
    }

    private async put<B>(url: string, body: B): Promise<Response> {
        const fullUrl = `${this.baseUrl}/${url}`;
        const init = {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body),
        };
        return await fetch(fullUrl, init);
    }

    private async delete(url: string): Promise<Response> {
        const fullUrl = `${this.baseUrl}/${url}`;
        const init = {
            method: "DELETE",
            headers: { "Content-Type": "application/json" }
        };
        return await fetch(fullUrl, init);
    }
}

export default ApiClient;
export { ApiError };