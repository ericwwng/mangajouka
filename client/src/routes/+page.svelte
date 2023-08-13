<script lang="ts">
    import Manga from "$lib/Manga.svelte";
    import type { PageData } from "./$types";

    let page = 0;

    let mangas = [];
    fetchManga();

    async function fetchNewPage() {
        let newPage = await fetch(`http://0.0.0.0:8000/api/manga/${page}`).then((mangas) => mangas.json()) 
        mangas = mangas.concat(newPage.data);
        page++;
    }

    async function fetchManga() {
        await fetchNewPage();

        while (mangas.length < 20) {
            await fetchNewPage();
        }

        console.log(mangas);
    }
</script>

<h1 class="h1 text-center">Manga Jouka</h1>
<div class="container mx-auto mt-16">
    <div class="space-y-4">
        {#each mangas as manga}
            <Manga manga={manga}/>
        {/each}
    </div>

    <div class="flex items-center justify-center p-4 rounded-lg gap-4">
        <button type="button" class="btn variant-filled" on:click={fetchManga}> New Page </button>
    </div>
</div>
