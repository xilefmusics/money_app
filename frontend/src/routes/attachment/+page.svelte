<script>
    import url2params from '../../lib/url/url2params';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';

    let name = null;
    let extension = null;
    const reload = async () => {
        const params = url2params($page.url.href)
        name = params.name
        const split = name.split(".")
        extension = split[split.length-1]
    }

    onMount(reload);
</script>

{#if name}
    {#if extension === "pdf"}
        <iframe title={name} src="/api/attachment/{name}" height="100%" width="100%"></iframe>
    {:else if extension === "jpg" || extension === "jpeg" || extension === "png"}
        <img src="/api/attachment/{name}" alt="{name}" width="100%"/>
        <form method="get" action="/api/attachment/{name}">
            <button type="submit">download {name}</button>
        </form>
     {:else}
        <form method="get" action="/api/attachment/{name}">
            <button type="submit">download {name}</button>
        </form>
    {/if}
{/if}

<style>
    button {
        background: none;
	    color: inherit;
	    border: none;
	    padding: 1em;
	    font: inherit;
	    cursor: pointer;
	    outline: inherit;
    }
    button:hover {
        background-color: var(--primary);
    }
</style>