import { PropsWithChildren, createContext, useContext, useEffect, useState } from "react";
import { Account } from "../model/account";
import { STORAGE_ACCOUNT } from "../utils/constants";

const AccountContext = createContext<AccountState>({
    account: null,
    setAccount: () => {}
});

function loadAccount(): Account | null {
    const accountString = localStorage.getItem(STORAGE_ACCOUNT);
    if(!accountString) {
        return null;
    }
    return JSON.parse(accountString) as Account;
}

/**
 * An {@link Account}, and a setter for an {@link Account}.
 */
export interface AccountState {
    account: Account | null,
    setAccount: (account: Account | null) => void,
};

/**
 * Hook for an {@link AccountState}.
 */
export function useAccountState(): AccountState {
    return useContext(AccountContext);
}

/**
 * Provides an {@link AccountState} to descendants.
 */
export default function AccountProvider(props: PropsWithChildren<{}>) {
    const [account, setAccount] = useState<Account | null>(loadAccount);
    useEffect(() => {
        if(account) {
            localStorage.setItem(STORAGE_ACCOUNT, JSON.stringify(account));
        }
        else {
            localStorage.removeItem(STORAGE_ACCOUNT);
        }
    }, [account]);
    const state: AccountState = { account, setAccount };
    return <AccountContext.Provider value={state}>{props.children}</AccountContext.Provider>
}