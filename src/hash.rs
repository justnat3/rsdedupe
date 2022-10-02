mod hash {

    struct FileMap {
        map: HashMap<String, PathBuf>,
        file_vector: Vec<PathBuf>,
        matches: i64,
    }

    struct IndexedFile {
        hash: &str,
        path: PathBuf,
    }

    impl FileMap {
        fn new(files: Vec<PathBuf>) -> Self {
            Self {
                map: HashMap::new(),
                matches: 0,
                file_vector: files,
            }
        }

        // fn index_files(&self) {
        //     for entry in &self.file_vector {
        //         let hash = self.hash_at_path(entry.to_path_buf());

        //         if let Some(hash) = hash {
        //             self.maybe_insert_hash(hash.as_str());
        //         }
        //     }
        // }

        // grab hash of a path
        fn hash_at_path(&self, path: &PathBuf) -> Option<IndexedFile> {
            let path = path.to_path_buf();
            let bytes = read(path);

            match bytes {
                Ok(v) => {
                    let file_bytes = Some(v);
                    let hash = sha256::digest_bytes(&v);
                    let i_file = Some(IndexedFile {hash, path});
                    return i_file
                },
                Err(_) => None,
            };
        }

        // Here we figure out whether the 
        fn maybe_insert_hash(&self, hash: &str) {
            if self.map.contains_key(hash) {}
        }
        // check for hash collisions
        fn differs(&self) {
            todo!()
        }
    }

    fn dummy_func() {
        // initialize map with some files to roll through
        let mut paths: Vec<PathBuf> = Vec::new();
        if let Ok(i) = std::fs::read_dir("./dummy_dir") {
            for entry in i {
                match entry {
                    Ok(v) => paths.push(v.path()),
                    Err(_) => todo!(),
                }
            }
        };

        let map = FileMap::new(paths);

        map.index_files();

        // pop file from stack
    }

}
