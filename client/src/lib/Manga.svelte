<script lang="ts">
    export let manga;

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
</script>


<div class="flex items-center p-4 bg-surface-800 rounded-lg gap-4"> 
    {#await coverArtUrlPromise then coverArtUrl}
        <!-- TODO: Image source should not hotlink mangadex -->
        <img class="h-48 w-full object-cover md:h-full md:w-48" src={coverArtUrl} alt="{manga.attributes.title} cover" />
    {/await}
    <h1>{manga.attributes.title.en}</h1>
    <p>{manga.attributes.description.en}</p>
</div>
