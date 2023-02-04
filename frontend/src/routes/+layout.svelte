<script>
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import getUser from '../lib/api/getUser';

	let isOpen = true;
	let innerWidth = 0;
	let innerHeight = 0;

	const navItems = [
		{
			icon: 'grid_view',
			text: 'Dashboard',
			href: '/dashboard'
		},
		{
			icon: 'list_alt',
			text: 'Transactions',
			href: '/transactions'
		},
		{
			icon: 'wallet',
			text: 'Pods',
			href: '/pods'
		},
		{
			icon: 'pie_chart',
			text: 'Budgets',
			href: '/budgets'
		},
		{
			icon: 'credit_card_off',
			text: 'Debts',
			href: '/debts'
		},
		{
			icon: 'logout',
			text: 'Logout',
			href: '/logout'
		}
	];

	onMount(async () => {
		if ($page.url.pathname !== '/login' && !getUser()) {
			goto('/login');
		}
	});
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<header>
	<div class="header-left">
		<span class="material-icons-sharp" on:click={() => (isOpen = !isOpen)}>menu</span>
	</div>
	<div class="header-center" />
	<div class="header-right">
		<span class="material-icons-sharp">account_circle</span>
	</div>
</header>

<main
	class={innerHeight > innerWidth
		? isOpen
			? 'mobileOpen'
			: 'mobileClosed'
		: isOpen
		? 'desktopOpen'
		: 'desktopClosed'}
>
	<slot />
</main>

<aside
	class={innerHeight > innerWidth
		? isOpen
			? 'mobileOpen'
			: 'mobileClosed'
		: isOpen
		? 'desktopOpen'
		: 'desktopClosed'}
>
	<div class="aside-top">
		<span class="material-icons-sharp" on:click={() => (isOpen = !isOpen)}>close</span>
	</div>
	<div class="aside-main">
		<nav>
			<ul>
				{#each navItems as item}
					<li class={item.href === $page.url.pathname || item.href === "/dashboard" && $page.url.pathname === "/" ? 'selected' : ''}>
						<a href={item.href}>
							<span class="material-icons-sharp">{item.icon}</span>
							<span class="text">{item.text}</span>
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	</div>
</aside>

<style>
	header {
		width: 100vw;
		height: var(--header-height);
		position: absolute;
		top: 0;
		left: 0;
		box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
		background-color: var(--bg2);
		display: flex;
	}

	header .header-left span {
		margin-top: 1.5rem;
		margin-left: 1.5rem;
		font-size: 2rem;
	}

	header .header-left span:hover {
		color: var(--primary);
	}

	header .header-center {
		flex: 1;
	}

	header .header-right span {
		margin-top: 1.5rem;
		margin-right: 2rem;
		font-size: 2rem;
	}

	aside {
		width: var(--aside-width);
		height: calc(100vh - var(--header-height));
		position: absolute;
		top: var(--header-height);
		left: 0;
		display: flex;
		flex-direction: column;
		transition: all 300ms ease;
		box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
		background-color: var(--bg2);
		padding: 0 1.5rem 0 0.5rem;
	}
	aside.desktopClosed {
		width: var(--header-height);
	}
	aside.mobileClosed {
		display: none;
	}
	aside.mobileOpen {
		height: 100vh;
		top: 0;
	}

	aside .aside-top {
		display: none;
		width: 100%;
	}
	aside.mobileOpen .aside-top {
		display: block;
	}

	aside .aside-main {
		width: 100%;
		height: 100%;
	}

	aside .aside-main nav ul {
		list-style: none;
	}

	aside .aside-main nav ul li {
		margin: 2rem 0 2rem 0;
		padding: 1rem 0 1rem 0;
		border-radius: 1rem;
	}

	aside .aside-main nav ul li:last-child {
		position: absolute;
		bottom: 1rem;
		width: calc(100% - 4rem);
	}

	aside .aside-main nav ul li:hover {
		margin-left: 1rem;
	}

	aside .aside-main nav ul li:hover a span {
		color: var(--primary);
	}

	aside .aside-main nav ul li.selected {
		margin-left: 1rem;
		background-color: var(--bg1);
	}

	aside.desktopClosed .aside-main nav ul li.selected,
	aside.desktopClosed .aside-main nav ul li:hover {
		margin-left: 0.5rem;
	}

	aside .aside-main nav ul li.selected a .text,
	aside .aside-main nav ul li.selected a .material-icons-sharp {
		color: var(--primary);
	}

	aside .aside-main ul li a {
		text-decoration: none;
	}

	aside .aside-main ul li a .text {
		font-size: 1.2rem;
		font-weight: 800;
	}

	aside .aside-main ul li a .material-icons-sharp {
		font-size: 2rem;
		padding: 0 1rem 0 1rem;
	}

	aside.desktopClosed .aside-main nav ul li a .text {
		display: none;
	}

	main {
		width: calc(100vw - var(--aside-width));
		height: calc(100vh - var(--header-height));
		position: absolute;
		top: var(--header-height);
		left: var(--aside-width);
		transition: all 300ms ease;
		padding: 1rem;
		overflow: scroll;
	}
	main.desktopClosed {
		width: calc(100vw - var(--header-height));
		left: var(--header-height);
	}
	main.mobileClosed {
		width: 100vw;
		left: 0;
	}
	main.mobileOpen {
		width: 100vw;
		left: 0;
	}
</style>
