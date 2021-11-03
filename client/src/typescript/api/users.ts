import {report_fetch_error} from "../error";
import {route} from "../server";
import type {Token} from "../token";

export interface User {
    id: string | null
    username: string
    display_name: string
    admin: boolean
    recipes: string[]
}

function parse_id(user: User): User {
    user.id = user["_id"]["$oid"]
    return user
}

export interface UpdateUser {
    display_name?: string
    admin?: boolean
    password?: string
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
    const resp = await fetch(route("/users/register"), {
        method: "POST",
        headers: new Headers({'content-type': 'application/json'}),
        body: JSON.stringify({
            "username": username,
            "password": password
        }, )
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


export async function me(token: Token): Promise<User | null> {
    const resp = await fetch(route("/users/me"), {
        method: "GET",
        headers: new Headers({
            'authorization': `Bearer: ${token}`,
        }),
    });

    if (!resp.ok) {
        await report_fetch_error(resp);
        return null;
    } else {
        const json = await resp.json();


        if (json === null || json === {}) {
            return null
        } else {
            return parse_id(json);
        }
    }
}

export async function update(token: Token, id: string, user: UpdateUser): Promise<boolean> {
    const resp = await fetch(route(`/users/${id}`), {
        method: "PATCH",
        headers: new Headers({
            'authorization': `Bearer: ${token}`,
            'content-type': 'application/json'
        }),
        body: JSON.stringify(user)
    });

    if (!resp.ok) {
        await report_fetch_error(resp);
        return false;
    } else {
        return true;
    }
}