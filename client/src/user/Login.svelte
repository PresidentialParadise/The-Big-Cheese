<script lang="ts">
    import {login} from "../typescript/api/users";
    import {loginToken} from "../typescript/token";
    import {useLocation, useNavigate} from "svelte-navigator";

    let username: string = "";
    let password: string = "";

    let loggingIn: boolean = false;

    const navigate = useNavigate();
    const location = useLocation();

    async function doLogin() {
        loggingIn = true;

        let token = await login(username, password);

        loggingIn = false;

        if (token !== null) {
            $loginToken = token;

            const from = ($location.state && $location.state.from) || "/";
            navigate(from, { replace: true });
        }
    }

    async function handleRegister() {
        navigate("/register")
    }
</script>

<div class="flex flex-col justify-center items-center w-full h-full">

    <form class="p-10 rounded shadow bg-c-gray-1 text-white" on:submit|preventDefault={doLogin}>
        <h1 class="text-3xl text-center pb-4 ">Login</h1>
        <div class="grid grid-cols-3 gap-4">
            <label for="username" class="flex flex-col justify-center"><span>Username</span></label>
            <input tabindex="1"
                id="username"
                name="username"
                type="text"
                class="p-2 col-span-2 bg-c-gray-2 border-none"
                bind:value={username}
            >
            <label for="password" class="flex flex-col justify-center"><span>Password</span></label>
            <input tabindex="2"
                id="password"
                name="password"
                type="password"
                class="p-2 col-span-2 bg-c-gray-2 border-none"
                bind:value={password}
            >

            {#if !loggingIn}
            <button tabindex="4" class="py-2 text-xl rounded bg-c-yellow-2 text-black border-none text-center" on:click|stopPropagation={handleRegister}>Register</button>
            <button tabindex="3" class="col-span-2 py-2 text-xl rounded bg-c-yellow-2 text-black border-none text-center">Log In</button>
            {:else}
                <div class="col-span-3 py-2 text-xl rounded bg-c-yellow-2 text-black border-none text-center">Authenticating...</div>
            {/if}
        </div>
    </form>
</div>