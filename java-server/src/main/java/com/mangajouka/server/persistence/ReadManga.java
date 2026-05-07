package com.mangajouka.server.persistence;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import org.hibernate.annotations.CreationTimestamp;
import org.hibernate.annotations.UpdateTimestamp;

import java.time.OffsetDateTime;
import java.util.UUID;

@Entity
@Table(name = "read_mangas")
public class ReadManga {

    @Id
    private UUID id = UUID.randomUUID();

    @Column(name = "user_id", nullable = false)
    private UUID userId;

    @Column(name = "manga_id", nullable = false)
    private String mangaId;

    private Integer rating;

    @Column(name = "created_at")
    @CreationTimestamp
    private OffsetDateTime createdAt;

    @Column(name = "updated_at")
    @UpdateTimestamp
    private OffsetDateTime updatedAt;

    // Required by Hibernate
    protected ReadManga() {}

    public ReadManga(final UUID userId,
                     final String mangaId,
                     final Integer rating) {
        this.userId = userId;
        this.mangaId = mangaId;
        this.rating = rating;
    }
}