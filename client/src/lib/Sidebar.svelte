<script lang="ts">
    import { page } from '$app/stores';
    import { AppRail, AppRailTile, AppRailAnchor } from '@skeletonlabs/skeleton';
    let currentTile: number = 0; 

</script>

<div class="grid grid-cols-[auto_1fr] h-full bg-surface-50-900-token border-r
    border-surface-500/30 {$$props.class ?? ''}">
    <AppRail background="bg-transparent" border="border-r border-surface-500/30">
        <AppRailAnchor href="/" selected={$page.url.pathname === '/'}>
            <h4 class="h4">Home</h4>
        </AppRailAnchor>
        <AppRailAnchor href="/filtered" selected={$page.url.pathname === '/filtered'}>
            Filtered Manga
        </AppRailAnchor>
    </AppRail>
</div>
