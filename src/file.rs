use crate::Error;
use std::fs;

/// Either file or directory
#[derive(Clone, Debug)]
pub enum FileDir {
    Dir(String, Vec<FileDir>, fs::Metadata),
    File(String, fs::Metadata),
}

// FileDir implementation
impl FileDir {
    /// List all files and directories recursively
    pub fn list_dir(dir: fs::ReadDir) -> Result<Vec<FileDir>, Error> {
        // create buffer
        let mut files = Vec::new();

        // loop through files in directory
        for file_res in dir {
            // get file
            let file: fs::DirEntry = match file_res {
                Ok(file) => file,
                Err(err) => return Error::from(err),
            };

            // get file type
            let file_type = match file.file_type() {
                Ok(file_type) => file_type,
                Err(err) => return Error::from(err),
            };

            // get file path
            let file_path = match file.path().to_str() {
                Some(file_name) => file_name.to_string(),
                None => return Error::from("File name does not contain valid UTF-8"),
            };

            // get file metadata
            let metadata = match file.metadata() {
                Ok(metadata) => metadata,
                Err(err) => return Error::from(err),
            };

            // check if is dir
            if file_type.is_dir() {
                // read directory
                let file_dir = match fs::read_dir(file.path()) {
                    Ok(file_dir) => file_dir,
                    Err(err) => return Error::from(err),
                };

                // add other files
                let sub_dir = match Self::list_dir(file_dir) {
                    Ok(sub_dir) => sub_dir,
                    Err(err) => return Error::from(err),
                };

                // add directory to list
                files.push(FileDir::Dir(file_path, sub_dir, metadata));
            } else {
                // add file to list
                files.push(FileDir::File(file_path, metadata));
            }
        }

        // return paths
        Ok(files)
    }
}
