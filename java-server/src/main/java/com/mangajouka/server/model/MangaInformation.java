package com.mangajouka.server.model;

import lombok.Data;

import java.util.List;

@Data
public class MangaInformation {
    private String id;
    private String title;
    private String description;
    private List<String> tags;
}
