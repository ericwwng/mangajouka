import type { PageLoad } from "./$types";
import axios from 'axios';

enum FilterStatus {
    NO_FILTER,
    INCLUDE,
    EXCLUDE
}

async function getTags() {
    const baseUrl = 'https://api.mangadex.org';
    //let tags = await fetch(`https://api.mangadex.org/manga/tag`);
    const resp = await axios({
        method: 'GET',
        url: `${baseUrl}/manga/tag`,
        params: {}
    }); 

    let tags = resp.data.data;
    for (let i = 0; i < tags.length; i++) {
        tags[i].filterStatus = FilterStatus.NO_FILTER; 
    }

    tags.sort((a, b) => {
        const nameA = a.attributes.name.en.toUpperCase(); 
        const nameB = b.attributes.name.en.toUpperCase(); 
        if (nameA < nameB) {
            return -1;
        }
        if (nameA > nameB) {
            return 1;
        }

        return 0;
    });

    return tags;
}

export const load: PageLoad = async () => {
    return {
        tags: getTags()
    };
};
