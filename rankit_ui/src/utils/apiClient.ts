import * as account from "../model/account";

class ApiError extends Error {}

class ApiClient {

    constructor(private baseUrl: string) {}

    async createAccount(request: account.CreateRequest): Promise<account.CreateResponse> {
        const response = await this.post("account", request);
        if(!response.ok) {
            throw new ApiError("Failed to create an account");
        }
        return response.json();
    }

    private async get(url: string): Promise<Response> {
        const fullUrl = `${this.baseUrl}/${url}`;
        const init = {
            method: "GET",
            headers: { "Content-Type": "application/json" },
        };
        return await fetch(fullUrl, init);
    }

    private async post<B>(url: string, body: B): Promise<Response> {
        const fullUrl = `${this.baseUrl}/${url}`;
        const init = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body),
        };
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