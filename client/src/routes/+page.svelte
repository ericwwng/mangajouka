<script lang="ts">
    import Manga from "$lib/Manga.svelte";
    import type { PageData } from "./$types";
    import axios from 'axios';

    export let data;
    let filtered_mangas = data.filtered_mangas;
    let tags = data.tags.data;

    for(let i = 0; i < tags.length; i++) {
        if (tags[i].attributes.name.en == "Fantasy") {
            console.log(tags[i]);
        }
    }

    let page = 0;

    let mangas = [];
    fetchManga();

    async function fetchNewPage() {
        const baseUrl = 'https://api.mangadex.org';
        const limit = 10;
        const resp = await axios({
            method: 'GET',
            url: `${baseUrl}/manga`,
            params: {
                // TODO: use user-selected tags instead of hardcoded string
                'includedTags[]': "cdc58593-87dd-415e-bbc0-2ec27bf404cc",
                'includes[]': 'cover_art',
                'order[rating]': 'desc',
                'limit': limit,
                'offset': page * limit
            }
        }); 

        let non_filtered_mangas = resp.data.data.filter((manga) => !filtered_mangas.has(manga.id));

        mangas = mangas.concat(non_filtered_mangas);
        page++;
    }

    async function fetchManga() {
        while (mangas.length < 10) {
            await fetchNewPage();
        }
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
