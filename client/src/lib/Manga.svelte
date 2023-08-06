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
</script>


<div class="flex items-center p-4 bg-surface-800 rounded-lg gap-4"> 
    {#if visible}
        {#await coverArtUrlPromise then coverArtUrl}
            <img class="h-48 w-full object-cover md:h-full md:w-48" src={coverArtUrl} alt="{manga.attributes.title} cover" />
        {/await}
        <h1>{manga.attributes.title.en}</h1>
        <p>{manga.attributes.description.en}</p>
        <button type="button" class="btn variant-filled" on:click={addFilteredManga(manga)}>Filter</button>
    {/if}
</div>
