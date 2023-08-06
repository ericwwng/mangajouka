<script lang="ts">
    import Manga from "$lib/Manga.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;
    let mangas = data.mangas.data;

    let page = 0;

    async function fetchManga() {
        page++;
        let newPage = await fetch(`http://0.0.0.0:8000/api/manga/${page}`).then((mangas) => mangas.json()) 
        mangas = mangas.concat(newPage.data);

        while (mangas.length < 20) {
            page++;
            let newPage = await fetch(`http://0.0.0.0:8000/api/manga/${page}`).then((mangas) => mangas.json()) 
            mangas = mangas.concat(newPage.data);
        }
    }
</script>

<div class="contained mx-auto mt-16">
    <h1 class="h1 text-center">Manga Jouka</h1>

    <div class="space-y-4">
        {#each mangas as manga}
            <Manga manga={manga}/>
        {/each}
    </div>

    <div class="flex items-center justify-center p-4 rounded-lg gap-4">
        <button type="button" class="btn variant-filled" on:click={fetchManga}> New Page </button>
    </div>
</div>
