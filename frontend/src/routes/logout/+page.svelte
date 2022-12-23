<script>
    import logout from '../../lib/api/logout';
    import { onMount } from 'svelte';
    import { goto } from "$app/navigation";

    onMount(() => {
        logout()
        goto("/login")
    })
</script>
<h1>Logout</h1>
