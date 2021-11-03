<script lang="ts">
    import {me, update, User} from "../typescript/api/users";
    import {loginToken} from "../typescript/token";
    import ChangeableInput from "../components/ChangeableInput.svelte";

    let user: User | null = null;

    async function fetchUser(): Promise<void> {
        user = await me($loginToken);
    }

    async function changeDisplayName(newName): Promise<string> {
        console.log(user);
        if (!await update($loginToken, user.id, {display_name: newName})) {
            return user.display_name
        } else {
            return newName
        }
    }

    async function changePassword(newPassword): Promise<string> {
        console.log(user);
        if (!await update($loginToken, user.id, {password: newPassword})) {
            return user.display_name
        } else {
            return newPassword
        }
    }
</script>

<div class="flex flex-col items-center w-full p-5">
    {#await fetchUser()}
        Loading
    {:then _}
        <div class="container bg-c-yellow-1 text-black md:w-3/4 w-full p-10 grid grid-cols-2 gap-4">
            <span class="flex flex-col justify-center">Username:</span>
            <ChangeableInput
                    value="{user.username}"
                    input_class="bg-c-yellow-2 p-2 text-black border-none"
                    readonly={true}
            />

            <span class="flex flex-col justify-center">Display Name:</span>
            <ChangeableInput
                    value="{user.display_name}"
                    input_class="bg-c-yellow-2 p-2 text-black border-none"
                    on_save={changeDisplayName}
            />

            <span class="flex flex-col justify-center">Password:</span>
            <ChangeableInput
                    value=""
                    type="password"
                    input_class="bg-c-yellow-2 p-2 text-black border-none"
                    on_save={changePassword}
            />
        </div>
    {/await}
</div>