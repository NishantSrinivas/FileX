use std::path::Path;
use std::fs;
pub fn count_occurrence(target: String, file_path: String) -> Option<u128> {
    let file_path = Path::new(&file_path);

    if !file_path.exists() {
        return None;
    }

    let mut count:u128 = 0;
    let contents = fs::read_to_string(file_path).
        expect("Unexpected error while reading file");

    for line in contents.lines() {
        let l = line.to_string();
        for word in l.split(' ') {
            if word == target {
                count += 1;
            }
        }
    }

    Some(count)
}