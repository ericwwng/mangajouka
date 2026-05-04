package com.mangajouka.server;

import com.mangajouka.server.persistence.MangaInformationEntity;
import com.mangajouka.server.persistence.MangaRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;
import java.util.UUID;

@RestController
public class MangaController {

    @Autowired
    private MangaRepository mangaRepository;

    @GetMapping("/manga")
    @ResponseBody
    public List<MangaInformationEntity> getManga(@RequestParam UUID userId) {
        return mangaRepository.findMangasByUserId(userId);
    }
}