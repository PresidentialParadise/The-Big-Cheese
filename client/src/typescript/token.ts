import {Writable, writable} from "svelte/store";

export type Token = string;

const LocalStorageTokenKey = "token"
function getToken(): Token | null {
    return window.localStorage.getItem(LocalStorageTokenKey)
}


export const loginToken: Writable<Token | null> = writable(getToken());
export const loggedIn: Writable<boolean> = writable(false);

loginToken.subscribe(v => {
    loggedIn.set(v !== null)

    if (v === null) {
        console.log("remove")
        window.localStorage.removeItem(LocalStorageTokenKey);
    } else {
        console.log("set")
        window.localStorage.setItem(LocalStorageTokenKey, v);
    }
})

