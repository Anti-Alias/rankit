import { Context, PropsWithChildren, createContext, useContext } from "react";
import ApiClient from "../utils/apiClient";

const AppDataContext: Context<AppData> = createContext<AppData>({ client: new ApiClient("") });

/**
 * Stores global and immutable application data.
 * Great for services and app-wide parameters.
 */
interface AppData {
    client: ApiClient
}

/**
 * @returns {@link AppData} provided by a {@link AppDataProvider}.
 */
function useApiClient(): ApiClient {
    return useContext(AppDataContext).client;
}

/**
 * Provides a {@link AppData} based on parameters stored in a .env file.
 */
function AppDataProvider({children}: PropsWithChildren<{}>) {
    const apiBaseUrl = process.env.REACT_API_API_BASE_URL;
    if(!apiBaseUrl) {
        throw new Error("Env var REACT_API_API_BASE_URL not set");
    }
    const appData = {
        client: new ApiClient(apiBaseUrl)
    }
    return <AppDataContext.Provider value={appData}>{children}</AppDataContext.Provider>;
}

export { useApiClient };
export default AppDataProvider;