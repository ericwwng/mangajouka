<script lang="ts">
    import Manga from "$lib/Manga.svelte";
    import FilterModal from "$lib/FilterModal.svelte";
    import type { PageData } from "./$types";
    import axios from 'axios';
	import { modalStore } from '@skeletonlabs/skeleton';
    import { loggedIn, userId } from '$lib/store.js';

    enum FilterStatus {
        NO_FILTER,
        INCLUDE,
        EXCLUDE
    }

    let loggedInValue;
    loggedIn.subscribe((value) => {
        loggedInValue = value;
    });

    let userIdValue;
    userId.subscribe((value) => {
        userIdValue = value;
    });

    export let data;
    let filtered_mangas;
    let tags = data.tags;
    let includedTags = [];
    let excludedTags = [];

    let page = 0;

    let mangas = [];
    let mangasMaxLength = 10;

    // To be used in testing
    let fetchCount = 0;
    let maxFetches = 10;
    if (loggedInValue) {
        reloadAfterFilterModal();
    }
    
    async function fetchRatings(non_filtered_mangas) {
        let manga_ids = [];
        for (let i = 0; i < non_filtered_mangas.length; i++) {
            manga_ids.push(non_filtered_mangas[i].id);
        }
        
        const baseUrl = 'https://api.mangadex.org';
        const limit = 10;
        const resp = await axios({
            method: 'GET',
            url: `${baseUrl}/statistics/manga`,
            params: {
                'manga[]': manga_ids
            }
        }); 

        return resp.data.statistics;
    }

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

        let ratings = await fetchRatings(non_filtered_mangas); 
        for (let i = 0; i < non_filtered_mangas.length; i++) {
            non_filtered_mangas[i].rating = ratings[non_filtered_mangas[i].id].rating.bayesian;

            // Some regex magic to truncate rating to 2 decimal points
            non_filtered_mangas[i].rating = non_filtered_mangas[i].rating.toString().match(/^-?\d+(?:\.\d{0,2})?/)[0];
        }

        mangas = mangas.concat(non_filtered_mangas);

        page++;
    }

    async function fetchManga() {
        while (mangas.length < mangasMaxLength) {
            fetchCount++;
            if (fetchCount >= maxFetches) {
                break;
            }
            await fetchNewPage();
        }
        mangasMaxLength += 10;
    }

    async function fetchFilteredMangas() { 
        const baseUrl = 'http://0.0.0.0:8000';
        const resp = await axios({
            method: 'GET',
            url: `${baseUrl}/api/filter`,
            params: {
                "user_id": userIdValue
            }
        }); 

        return new Set(resp.data);
    }

    async function reloadAfterFilterModal() {
        mangas = [];
        page = 0;
        filtered_mangas = await fetchFilteredMangas();

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

{#if loggedInValue}
    <div class="flex items-center justify-center p-4 rounded-lg gap-4">
        <button type="button" class="btn variant-filled" on:click={modalFilter}>Filter Settings</button>
    </div>

    <div class="space-y-4">
        {#each mangas as manga}
            <Manga manga={manga}/>
        {/each}
    </div>

    <div class="flex items-center justify-center p-4 rounded-lg gap-4">
        <button type="button" class="btn variant-filled" on:click={fetchManga}>New Page</button>
    </div>
{:else}
    <div class="flex items-center justify-center p-4 rounded-lg gap-4">
        <a href="/login" class="btn variant-filled">Login</a>
    </div>
{/if}
