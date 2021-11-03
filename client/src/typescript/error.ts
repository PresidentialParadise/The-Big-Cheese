import {navigate} from "svelte-navigator";
import {writable} from "svelte/store";

export const error_message = writable(null);
let timeout = null;

export function report_error(text: string) {
    console.error(text);
    error_message.set(text)
    if (timeout !== null) {
        clearTimeout(timeout)
    }
    if (timeout !== null) {
        clearTimeout(timeout);
    }
    timeout = setTimeout(() => {
        error_message.set(null)
        timeout = null;
    }, 8000)
}

export async function report_fetch_error(resp: Response, loginOnUnauthorized: boolean = true) {
    let text: string | null = await resp.text()
    if (text !== null && text !== "" && text !== "null") {
        report_error(text)
    } else if (resp.status >= 500 && resp.status < 600) {
        report_error("server error")
    } else if (resp.status === 401) {
        report_error("unauthorized")

        if (loginOnUnauthorized) {
            navigate("/login", {
                state: { from: location.pathname },
                replace: true,
            });
        }

    } else if (resp.status === 404) {
        report_error("not found")
    }else if (resp.status === 400) {
        report_error("bad request")
    } else {
        report_error("unknown error")
    }
}