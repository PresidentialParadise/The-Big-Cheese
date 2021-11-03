
<script lang="ts">
    import RecipePreview from "./components/RecipePreview.svelte";
    import type {Recipe} from "./typescript/api/recipe";
    import {get_all_recipes} from "./typescript/api/recipe";

    let recipes: Recipe[] = [];

    async function getRecipes() {
        const res = await get_all_recipes()
        if (res !== null) {
            recipes = res;
        }
    }

</script>

<div class="w-full h-full grid lg:grid-cols-4 md:grid-cols-2 sm:grid-cols-1 grid-auto-rows gap-10 px-5">
    {#await getRecipes() then _}
        {#each recipes as r}
            <RecipePreview name="{r.title}" />
        {/each}
    {/await}
</div>

