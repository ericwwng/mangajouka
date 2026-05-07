package com.mangajouka.server.graphql;

import com.mangajouka.server.persistence.ReadManga;
import com.mangajouka.server.persistence.ReadMangaRepository;
import com.netflix.graphql.dgs.DgsComponent;
import com.netflix.graphql.dgs.DgsMutation;
import com.netflix.graphql.dgs.DgsQuery;
import com.netflix.graphql.dgs.InputArgument;

import java.util.List;
import java.util.UUID;

@DgsComponent
public class ReadMangaDataFetcher {

    private final ReadMangaRepository readMangaRepository;

    public ReadMangaDataFetcher(final ReadMangaRepository readMangaRepository) {
        this.readMangaRepository = readMangaRepository;
    }

    @DgsQuery
    public List<ReadManga> readMangas(final @InputArgument String userId) {
        return readMangaRepository.findByUserId(UUID.fromString(userId));
    }

    @DgsQuery
    public ReadManga readManga(
            @InputArgument final String userId,
            @InputArgument final String mangaId
    ) {
        return readMangaRepository
                .findByUserIdAndMangaId(UUID.fromString(userId), mangaId)
                .orElse(null);
    }

    @DgsMutation
    public ReadManga addReadManga(
            @InputArgument final String userId,
            @InputArgument final String mangaId,
            @InputArgument final Integer rating
    ) {
        ReadManga entry = new ReadManga(
                UUID.fromString(userId),
                mangaId,
                rating
        );
        return readMangaRepository.save(entry);
    }

    @DgsMutation
    public boolean deleteReadManga(@InputArgument final String id) {
        UUID uuid = UUID.fromString(id);
        if (!readMangaRepository.existsById(uuid)) return false;
        readMangaRepository.deleteById(uuid);
        return true;
    }
}
