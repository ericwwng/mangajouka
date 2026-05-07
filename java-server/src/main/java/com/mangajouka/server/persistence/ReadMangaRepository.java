package com.mangajouka.server.persistence;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Repository
public interface ReadMangaRepository extends JpaRepository<ReadManga, UUID> {

    List<ReadManga> findByUserId(UUID userId);

    Optional<ReadManga> findByUserIdAndMangaId(UUID userId, String mangaId);
}
