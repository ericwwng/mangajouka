import type { PageLoad } from "./$types";
import axios from 'axios';

async function fetchFilteredMangas() { 
    const baseUrl = 'http://0.0.0.0:8000';
    const resp = await axios({
        method: 'GET',
        url: `${baseUrl}/api/filter`,
        params: {}
    }); 

    return new Set(resp.data);
}

async function getTags() {
    const baseUrl = 'https://api.mangadex.org';
    //let tags = await fetch(`https://api.mangadex.org/manga/tag`);
    const resp = await axios({
        method: 'GET',
        url: `${baseUrl}/manga/tag`,
        params: {}
    }); 

    return resp.data;
}

export const load: PageLoad = async () => {
    return {
        tags: getTags(),
        filtered_mangas: fetchFilteredMangas()
    };
};
