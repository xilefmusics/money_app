<script>
    import login from '../../lib/api/login';
    import { goto } from "$app/navigation";

    let username
    let password
</script>
<h1>Login</h1>

<input type="text" bind:value={username}>
<input type="password" bind:value={password}>
<button on:click={()=> {
    login(username,password)
    goto("/dashboard")
}}>Login</button>
