package com.mangajouka.server.persistence;

import jakarta.persistence.*;
import lombok.Getter;
import lombok.ToString;

@Entity
@Table(name = "manga_information")
@Getter
@ToString
public class MangaInformationEntity {

    @Id
    @Column(name = "manga_id", nullable = false)
    @GeneratedValue(strategy = GenerationType.UUID)
    private String mangaId;

    @Column(name = "manga_name")
    private String mangaName;

    @Column(name = "manga_description", columnDefinition = "TEXT")
    private String mangaDescription;

    @Column(name = "manga_tags")
    private String[] mangaTags;  // or List<String> if it's an array type
}
