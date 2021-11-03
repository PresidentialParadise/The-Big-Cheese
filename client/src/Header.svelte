<script>
    import {loggedIn, loginToken} from "./typescript/token";
    import {Link} from "svelte-navigator";

    function doLogOut() {
        $loginToken = null;
    }
</script>

<header class="top-0 fixed z-50 w-full flex flex-wrap items-center justify-between px-2 py-3 navbar-expand-lg bg-white shadow bg-c-yellow-1">
    <nav class="flex flex-row">
        <Link to="/" class="mx-2"><span class="text-c-gray-2 font-bold">Home</span></Link>
        {#if $loggedIn}
            <Link to="create" class="mx-2"><span class="text-c-gray-2 font-bold">Create Recipe</span></Link>
            <Link to="account" class="mx-2"><span class="text-c-gray-2 font-bold">Account</span></Link>
            <Link to="/" class="mx-2"><span class="text-c-gray-2 font-bold" on:click={doLogOut}>Logout</span></Link>
        {:else}
            <Link to="login" class="mx-2"><span class="text-c-gray-2 font-bold">Log In</span></Link>
        {/if}
    </nav>

    <div class="corner">
    </div>
</header>
