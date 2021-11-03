
<script lang="ts">

    import {create_recipe} from "./typescript/api/recipe";
    import {loginToken} from "./typescript/token";

    let title: string = "";
    let description: string = "";

    async function submitRecipe() {
        await create_recipe($loginToken, {
            title,
            description,
            servings: "",
            categories: [],
            cook_time: 0,
            ingredients: [],
            instructions: [],
            prep_time: 0,
            tags: []
        })
    }
</script>

<div class="flex flex-col items-center w-full p-5">

    <div class="container bg-c-yellow-1 text-black md:w-3/4 w-full p-10 grid grid-cols-4 gap-4">
        <label for="title" class="col-span-4 text-center"><span class="text-xl">Title</span></label>
        <input id="title" class="col-span-4 p-2 bg-c-yellow-2" bind:value={title}>

        <label for="description" class="col-span-4 text-center"><span class="text-xl">Description</span></label>
        <textarea id="description" class="col-span-4 p-2 bg-c-yellow-2" bind:value={description}></textarea>

        <button class="col-span-2 col-start-2 bg-c-red text-white p-3" on:click={submitRecipe}>Save</button>
    </div>
</div>