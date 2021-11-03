import {report_fetch_error} from "../error";
import {route, server_url} from "../server";
import type {Token} from "../token";

export enum Measurement {
    Kilogram,
    Gram,
    Milligram,

    Litre,
    Decilitre,
    Millilitre,

    Tablespoon,
    Teaspoon,

    Gallon,
    Quart,
    Pint,
    Cup
}

export interface Ingredient {
    title: string
    note: string
    quantity: Quantity
}

export interface Quantity {
    value: number,
    unit: Measurement,
}

export interface Recipe {
    id?: string | null
    title: string
    description: string
    servings: string

    ingredients: Ingredient[],
    instructions: string[]
    tags: string[]
    categories: string[]
    prep_time: number,
    cook_time: number,
}

export async function get_all_recipes(): Promise<Recipe[] | null> {
    const resp = await fetch(route("/recipes"), {
        method: "GET",
        headers: new Headers({'content-type': 'application/json'}),
    });

    if (!resp.ok) {
        await report_fetch_error(resp);
        return null;
    } else {
        const json = await resp.json();

        if (json === null || json === {}) {
            return null
        } else {
            return json
        }
    }
}

export async function create_recipe(token: Token, recipe: Recipe): Promise<void> {
    const resp = await fetch(route("/recipes"), {
        method: "POST",
        headers: new Headers({
            'content-type': 'application/json',
            'authorization': `Bearer: ${token}`,
        }),
        body: JSON.stringify(recipe)
    });

    if (!resp.ok) {
        await report_fetch_error(resp);
        return null;
    }
}

