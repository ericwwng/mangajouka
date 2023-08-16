<script lang="ts">
    export let parent: any;

	import { modalStore } from '@skeletonlabs/skeleton';

    enum FilterStatus {
        NO_FILTER,
        INCLUDE,
        EXCLUDE
    }

    let tags = $modalStore[0].tags;

	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cForm = 'border-surface-500 p-4 space-y-4';

    type FilterVariantMap = {
        [id: FilterStatus]: String;
    }

    const filterVariantMap: FilterVariantMap = {};
    filterVariantMap[FilterStatus.NO_FILTER] = 'variant-soft';
    filterVariantMap[FilterStatus.INCLUDE] = 'variant-filled';
    filterVariantMap[FilterStatus.EXCLUDE] = 'variant-filled-primary';

    function cycleFilterStatus(index) {
        if (tags[index].filterStatus === FilterStatus.NO_FILTER) {
            tags[index].filterStatus = FilterStatus.INCLUDE;
        } else if (tags[index].filterStatus === FilterStatus.INCLUDE) {
            tags[index].filterStatus = FilterStatus.EXCLUDE;
        } else {
            tags[index].filterStatus = FilterStatus.NO_FILTER;
        }        
    }
</script>

{#if $modalStore[0]}
    <div class="{cBase}">
        <form class="modal-form {cForm}">
            {#each tags as tag, i}
                <span class="chip {filterVariantMap[tag.filterStatus]}" 
                    on:click={() => {cycleFilterStatus(i)}}
                    on:keypress
                >
                    {tag.attributes.name.en}
                </span>
            {/each}
        </form>
    </div>
{/if}
