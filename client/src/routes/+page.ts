import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    return {
        mangas: (await fetch("http://0.0.0.0:8000/api/manga/0").then((mangas) => mangas.json()))
    };
};
