<script lang="ts">
    import Manga from "$lib/Manga.svelte";
    import FilterModal from "$lib/FilterModal.svelte";
    import type { PageData } from "./$types";
    import axios from 'axios';
	import { modalStore } from '@skeletonlabs/skeleton';

    enum FilterStatus {
        NO_FILTER,
        INCLUDE,
        EXCLUDE
    }

    export let data;
    let filtered_mangas = data.filtered_mangas;
    let tags = data.tags;
    let includedTags = [];
    let excludedTags = [];

    let page = 0;

    let mangas = [];
    let mangasMaxLength = 10;
    fetchManga();

    async function fetchNewPage() {
        const baseUrl = 'https://api.mangadex.org';
        const limit = 10;
        const resp = await axios({
            method: 'GET',
            url: `${baseUrl}/manga`,
            params: {
                'includedTags': includedTags,
                'excludedTags': excludedTags,
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
        while (mangas.length < mangasMaxLength) {
            await fetchNewPage();
        }
        mangasMaxLength += 10;
    }

    async function reloadAfterFilterModal() {
        mangas = [];
        page = 0;

        for(let i = 0; i < tags.length; i++) {
            if (tags[i].filterStatus === FilterStatus.INCLUDE) {
                includedTags.push(tags[i].id); 
            } else if (tags[i].filterStatus === FilterStatus.EXCLUDE) {
                excludedTags.push(tags[i].id);
            }
        }
        fetchManga();
    }

    function modalFilter() {
        const c: ModalComponent = { ref: FilterModal };
        const modal: ModalSettings = {
            type: 'component',
            component: c,
            tags: tags,
            response: () => reloadAfterFilterModal()
        };
        modalStore.trigger(modal);
    }
</script>

<h1 class="h1 text-center">Manga Jouka</h1>
<div class="flex items-center justify-center p-4 rounded-lg gap-4">
    <button type="button" class="btn variant-filled" on:click={modalFilter}>Filter</button>
</div>

<div class="space-y-4">
    {#each mangas as manga}
        <Manga manga={manga}/>
    {/each}
</div>

<div class="flex items-center justify-center p-4 rounded-lg gap-4">
    <button type="button" class="btn variant-filled" on:click={fetchManga}> New Page </button>
</div>
