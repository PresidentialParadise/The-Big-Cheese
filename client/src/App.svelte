<script lang="ts">
	import "styles/tailwind-output.css"
    import {Route, Router} from "svelte-navigator";
    import Header from "./Header.svelte";
    import Home from "./Home.svelte";
    import Footer from "./Footer.svelte";
    import {error_message} from "./typescript/error";
    import Login from "./user/Login.svelte";
    import Register from "./user/Register.svelte";
    import PrivateRoute from "./private/PrivateRoute.svelte";
    import Account from "./user/Account.svelte";
    import Create from "./Create.svelte";
</script>


<Router>
    <Header />

    <main class="header relative pt-16 items-center flex h-screen max-h-860-px bg-c-gray-2">
        <Route path="/">
            <Home />
        </Route>

        <Route path="/login">
            <Login />
        </Route>
        <Route path="/register">
            <Register />
        </Route>
        <PrivateRoute path="/account">
            <Account />
        </PrivateRoute>
        <PrivateRoute path="/create">
            <Create />
        </PrivateRoute>
    </main>

    <Footer />

    {#if $error_message !== null}
        <div class="fixed c-black p-4 bg-red-500 bottom-1 right-1">
            {$error_message}
        </div>
    {/if}
</Router>

<style global lang="postcss">
    @tailwind base;
    @tailwind components;
    @tailwind utilities;

    :global(body),:global(html) {
        margin: 0;
        padding: 0;

        color: white;
    }
</style>