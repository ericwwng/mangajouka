package com.mangajouka.server.persistence;

import jakarta.persistence.*;

import java.util.UUID;

@Entity
@Table(name = "filtered_mangas")
public class FilteredMangaEntity {

    @Id
    @Column(nullable = false, columnDefinition = "uuid")
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "user_id", nullable = false)
    private UserEntity user;

    @Column(name = "manga_id", nullable = false)
    private String mangaId;
}
