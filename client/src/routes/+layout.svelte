<script lang='ts'>
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';

    import { AppShell, AppBar, Modal } from '@skeletonlabs/skeleton';

    import Sidebar from '$lib/Sidebar.svelte';
</script>

<Modal />
<AppShell
    header="bg-surface-100/5 w-56 p-4">
    <slot />

    <svelte:fragment slot="header">
        <AppBar class="w-full">
            <svelte:fragment slot="lead">
                <a href="/"><h3 class="h3">MangaJouka</h3></a>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                <a href="/register">Register</a>
                <a href="/login">Login</a>
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

</AppShell>
