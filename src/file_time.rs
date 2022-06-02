use std::fs::Metadata;

use chrono::{DateTime, Utc};

pub struct FileTime {
    pub metadati_file: Metadata,
}

impl FileTime {
    pub fn new(metadati: Metadata) -> FileTime {
        //istanzio e restiuisco
        FileTime {
            metadati_file: metadati,
        }
    }

    pub fn get_mese(&self) -> i32 {
        let st = self.metadati_file.modified().unwrap();
        let dt: DateTime<Utc> = st.clone().into();
        let x = format!("{}", dt.format("%m"));
        x.trim().parse::<i32>().unwrap_or(0)
    }

    pub fn get_anno(&self) -> i32 {
        let st = self.metadati_file.modified().unwrap();
        let dt: DateTime<Utc> = st.clone().into();
        let x = format!("{}", dt.format("%Y"));
        x.trim().parse::<i32>().unwrap_or(0)
    }

    //creao una tupla da restituire con anno + mese
    pub fn get_anno_mese(&self) -> (i32, i32) {
        return (self.get_anno(), self.get_mese());
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_mese_file() {
        let metadati = fs::metadata(".\\Cargo.toml").unwrap();
        let file_time = FileTime::new(metadati);
        let mese = file_time.get_mese();
        println!("mese trovato {}", &mese);
        assert_eq!(mese, 6, "Test fallito perche non è il mese corrente");
    }

    #[test]
    fn test_anno_file() {
        let metadati = fs::metadata(".\\Cargo.toml").unwrap();
        let file_time = FileTime::new(metadati);
        let anno = file_time.get_anno();
        println!("anno trovato {}", &anno);
        assert_eq!(anno, 2022, "Test fallito perche non è il mese corrente");
    }
}
