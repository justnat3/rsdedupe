use std::fs::read;
use std::path::PathBuf;

struct FileMap {
    map: HashMap<String, PathBuf>,
    matches: i64,
    file_vector: Vec<PathBuf>,
}

impl FileMap {
    fn new(&self, files: Vec<DirEntry>) -> Self {
        Self {
            map: HashMap::new(),
            matches: 0,
            file_vector: Vec::new(),
        }
    }

    fn index_files(&self) { 
        for file in self.file_map.iter() {
            let entry = file?;
            let hash = hash_at_path(entry.path());
        }
    }

    // grab hash of a path
    fn hash_at_path(&self, path: PathBuf) {
        let bytes: Vec<u8> = read(path).unwrap();
        sha256::digest_bytes(&bytes);
    }
    // insert hash to the file map
    fn maybe_insert_hash(&self) {
        todo!()
    }
    // check for hash collisions
    fn differs(&self) {
        todo!()
    }
}

fn dummy_func() {
    // initialize map with some files to roll through
    if let Ok(i) = std::fs::read_dir("./dummy_dir") { 
        let map = FileMap::new(i);
    };



    // pop file from stack

}
