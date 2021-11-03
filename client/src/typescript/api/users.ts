import {report_fetch_error} from "../error";
import {server_url} from "../server";
import type {Token} from "../token";

function route(routename: string): string {
    return `${server_url}/${routename}`.replace("//", "/").replace("//", "/").replace("http:/", "http://")
}

export async function login(username: string, password: string): Promise<Token | null> {
    const resp = await fetch(route("/users/login"), {
        method: "POST",
        headers: new Headers({'content-type': 'application/json'}),
        body: JSON.stringify({
            "username": username,
            "password": password
        })
    });

    if (!resp.ok) {
        await report_fetch_error(resp);
        return null;
    } else {
        const json = await resp.json();

        if (json === null || json === {}) {
            return null
        } else {
            return json["token"]
        }
    }
}

export async function register(username: string, password: string): Promise<Token | null> {
    const resp = await fetch(`${server_url}/users/register`, {
        method: "POST",
        headers: new Headers({'content-type': 'application/json'}),
        body: JSON.stringify({
            "username": username,
            "password": password
        })
    });

    if (!resp.ok) {
        await report_fetch_error(resp);
        return null;
    } else {
        const json = await resp.json();

        if (json === null || json === {}) {
            return null
        } else {
            return json["token"]
        }
    }
}
