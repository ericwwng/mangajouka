<script lang="ts">
    import axios from 'axios';

    export let manga; 

    let visible = true;

    async function getCoverArtUrl() {
        let filename;
        for (let i = 0; i < manga.relationships.length; i++) {
            if (manga.relationships[i].type === "cover_art") {
                filename = manga.relationships[i].attributes.fileName;
            }
        }
        let res = await fetch(`http://0.0.0.0:8000/api/cover?manga_id=${manga.id}&filename=${filename}`);
        let url = await res.text();

        return url;
    }

    let coverArtUrlPromise = getCoverArtUrl();

    async function addFilteredManga(filtered_manga) {
        await fetch(`http://0.0.0.0:8000/api/filter`, {
            method: "POST",
            body: JSON.stringify({"manga_id": filtered_manga.id}),
           	headers: {
    			'Content-Type': 'application/json'
            }}
        );

        visible = false;
    }

    function getMangaName() {
        if ("en" in manga.attributes.title) {
            return manga.attributes.title["en"];
        }
        else if ("ja-ro" in manga.attributes.title) {
            return manga.attributes.title["ja-ro"];
        }
        return manga.attributes.title["ja"];
    }

    function getMangaNameWithStatus() {
        let mangaName = getMangaName();
        if (manga.attributes.status === "completed") {
            mangaName += " [COMPLETED]";
        }

        return mangaName;
    }

    async function getLatestChapter() {
        const baseUrl = 'https://api.mangadex.org';
        const limit = 10;
        const resp = await axios({
            method: 'GET',
            url: `${baseUrl}/chapter`,
            params: {
                'limit': 1,
                'manga': manga.id,
                'order[chapter]': 'desc'
            }
        }); 

        return resp.data.data[0].attributes.chapter;
    }

    let latestChapterPromise = getLatestChapter();
</script>


{#if visible}
    <div class="container mx-auto flex items-center p-4 bg-surface-600 rounded-lg gap-4">
        {#await coverArtUrlPromise then coverArtUrl}
            <img class="h-auto w-56 object-cover" src={coverArtUrl} alt="{manga.attributes.title} cover" />
        {/await}
        <div class="w-full p-4 space-y-4 preview-description">
            {#await latestChapterPromise then latestChapter}
                <div class="flex justify-between">
                    <div class="overflow-auto h-12">
                        <h3 class="h3">{getMangaNameWithStatus()}</h3>
                    </div>
                    <h3 class="h3">{latestChapter}</h3>
                </div>
            {/await}

            {#each manga.attributes.tags as tag} 
                <span class="tag">{tag.attributes.name.en}</span>
            {/each}

            <div class="overflow-auto h-32">
                {manga.attributes.description.en}
            </div>
            <button type="button" class="btn variant-filled-primary" on:click={addFilteredManga(manga)}>Filter</button>
        </div>
    </div>
{/if}
