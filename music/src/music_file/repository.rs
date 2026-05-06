pub struct MusicLibraryRepository;

impl MusicLibraryRepository {
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    pub fn load(
        dir: &std::path::Path,
    ) -> Result<crate::music_file::MusicLibrary, crate::music_file::MusicFileErrors>
    {
        tracing::debug!(
            "Loading monthly music files from directory: `{}`",
            dir.display()
        );

        let file_paths = Self::collect_music_file_paths_in_dir(dir);
        let (files, errs) = Self::load_music_files(file_paths, dir);

        if !errs.is_empty() {
            tracing::error!(
                "Loaded {} monthly music month files with {} errors from directory `{}`",
                files.len(),
                errs.len(),
                dir.display()
            );
            return Err(errs.into());
        }

        if files.is_empty() {
            return Err(crate::music_file::MusicFileError::InvalidPath {
                path: dir.to_path_buf(),
                msg: format!(
                    "No monthly music files found in directory `{}`",
                    dir.display()
                ),
            }
            .into_errors());
        }

        tracing::info!(
            "Loaded {} monthly music month files from directory `{}`",
            files.len(),
            dir.display()
        );

        Ok(crate::music_file::MusicLibrary::new(
            dir.to_path_buf(),
            files,
        ))
    }

    pub fn save_month_files(
        library: &crate::music_file::MusicLibrary,
    ) -> Result<(), crate::music_file::MusicFileErrors> {
        tracing::info!("Saving monthly music files to disk...");

        let mut errs = Vec::new();

        for file in library.iter_files() {
            if let Err(e) = file.save() {
                errs.push(crate::music_file::MusicFileError::FileWrite {
                    path: file.get_path().to_path_buf(),
                    msg: e.to_string(),
                });
            }
        }

        if errs.is_empty() {
            tracing::info!("Saved all monthly music files saved successfully.");
            Ok(())
        } else {
            Err(errs.into())
        }
    }

    fn collect_music_file_paths_in_dir(
        dir: &std::path::Path,
    ) -> Vec<std::path::PathBuf> {
        let mut file_paths = Vec::new();

        for entry in walkdir::WalkDir::new(dir) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            if entry.file_type().is_file() {
                let path = entry.path();
                if Self::is_monthly_music_file_path(dir, path) {
                    file_paths.push(path.to_path_buf());
                } else {
                    tracing::trace!(
                        "Skipped non-month music file while loading: {}",
                        path.display()
                    );
                }
            }
        }

        file_paths.sort_unstable();
        file_paths
    }

    fn is_monthly_music_file_path(
        root: &std::path::Path,
        path: &std::path::Path,
    ) -> bool {
        let rel = match path.strip_prefix(root) {
            Ok(rel) => rel,
            Err(_) => return false,
        };

        let mut components = rel.components();
        let year = match components.next().and_then(|c| c.as_os_str().to_str()) {
            Some(year) => year,
            None => return false,
        };
        let month_file = match components.next().and_then(|c| c.as_os_str().to_str()) {
            Some(month_file) => month_file,
            None => return false,
        };

        if components.next().is_some() {
            return false;
        }

        if year.len() != 4 || !year.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        if month_file.len() != 7 || !month_file.ends_with(".json") {
            return false;
        }

        match month_file[..2].parse::<usize>() {
            Ok(month) => (1..=12).contains(&month),
            Err(_) => false,
        }
    }

    fn load_music_files(
        file_paths: Vec<std::path::PathBuf>,
        dir: &std::path::Path,
    ) -> (
        std::collections::HashMap<(usize, usize), crate::music_file::MusicFile>,
        Vec<crate::music_file::MusicFileError>,
    ) {
        use std::collections::HashMap;

        let mut files: HashMap<(usize, usize), crate::music_file::MusicFile> =
            HashMap::new();
        let mut errs = Vec::new();

        for file_path in file_paths {
            match crate::music_file::MusicFile::load(file_path, dir) {
                Ok(music_file) => {
                    let (year, month) = music_file.get_year_month();
                    let duplicated_path = music_file.get_path().to_path_buf();

                    if let Some(existing) =
                        files.insert(music_file.get_year_month(), music_file)
                    {
                        errs.push(
                            crate::music_file::MusicFileError::DuplicateYearMonthFile {
                                year,
                                month,
                                existing_path: existing.get_path().to_path_buf(),
                                duplicated_path,
                            },
                        );
                    }
                }
                Err(error) => errs.push(error),
            }
        }

        (files, errs)
    }
}
