<script>
    import { useNavigate, useLocation } from "svelte-navigator";
    import {loggedIn} from "../typescript/token";

    const navigate = useNavigate();
    const location = useLocation();

    $: if (!$loggedIn) {
        navigate("/login", {
            state: { from: $location.pathname },
            replace: true,
        });
    }
</script>

{#if $loggedIn}
    <slot />
{/if}
