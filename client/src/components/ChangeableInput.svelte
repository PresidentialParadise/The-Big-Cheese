<script lang="ts">
    import Fa from "svelte-fa";

    export let type: string = "text";
    export let value: string = "";
    export let input_class: string = "";
    export let button_class: string = "";

    export let readonly: boolean = false;

    export let on_save: (string) => Promise<string> = async a => a;

    import {faTrash, faPen, faSave} from '@fortawesome/free-solid-svg-icons'

    let internalValue: string = value;

    let editing: boolean = false;
    let savedValue: string = "";

    let input;

    function handleEdit() {
        savedValue = internalValue;
        editing = true;

        input.focus();
    }

    async function handleSave() {
        editing = false;
        value = await on_save(internalValue)
    }

    function handleDiscard() {
        editing = false;
        internalValue = savedValue;
    }

</script>

<div class="flex flex-row justify-between">
    {#if type === "password"}
        <input type=password bind:this={input} class="{input_class}" bind:value={internalValue} readonly="{!editing || null}" placeholder="********">
    {:else}
        <input bind:this={input} class="{input_class}" bind:value={internalValue} readonly="{!editing || null}">
    {/if}
    {#if editing}
        <button class="{button_class} border-none" on:click={handleSave}>
            <Fa icon={faSave}/>
        </button>
        <button class="{button_class} border-none" on:click={handleDiscard}>
            <Fa icon={faTrash}/>
        </button>
    {:else if !readonly}
        <button class="{button_class} border-none" on:click={handleEdit}>
            <Fa icon={faPen}/>
        </button>
    {/if}
</div>