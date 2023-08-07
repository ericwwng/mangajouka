<script lang="ts">
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
</script>


{#if visible}
    <div class="flex items-center p-4 bg-surface-600 rounded-lg gap-4">
        {#await coverArtUrlPromise then coverArtUrl}
            <img class="h-auto w-48 object-cover" src={coverArtUrl} alt="{manga.attributes.title} cover" />
        {/await}
        <div class="p-4 space-y-4 preview-description">
            <h3 class="h3">{getMangaName()}</h3>

            {#each manga.attributes.tags as tag} 
                <span class="tag">{tag.attributes.name.en}</span>
            {/each}
            <div class="overflow-auto h-24">
                {manga.attributes.description.en}
            </div>
            <button type="button" class="btn variant-filled-primary" on:click={addFilteredManga(manga)}>Filter</button>
        </div>
    </div>
{/if}
