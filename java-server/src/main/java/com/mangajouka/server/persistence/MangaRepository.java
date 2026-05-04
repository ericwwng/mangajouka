package com.mangajouka.server.persistence;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface MangaRepository extends JpaRepository<MangaInformationEntity, Long> {
    @Query("""
        SELECT mi FROM MangaInformationEntity mi
        JOIN FilteredMangaEntity fm ON mi.mangaId = fm.mangaId
        WHERE fm.user.id = :userId
        """)
    List<MangaInformationEntity> findMangasByUserId(@Param("userId") UUID userId);
}
