use file_time::FileTime;
use std::{fs, io::Error, process::Command, path::Path};

use clap::Parser;

//importare un tuo file
mod file_time;


//COSTANTI PATH ARRIVO E PARTENZA
const FILE_PARTENZA: &str = "C:\\CASA\\PROVA_RUST\\rust_comprimi_mese\\resources\\paths_Partenza.txt";
const FILE_ARRIVO: &str = "C:\\CASA\\PROVA_RUST\\rust_comprimi_mese\\resources\\path_Arrivo.txt";

/// Simple app for backup files and folders recursively from a file with a list of paths
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Argomenti {
    // short: il parametro corto (-i), long: il parametro con nome completo --input-file-with-paths
    #[clap(short = 'i', long)]
    input_path: String,
    #[clap(short = 'o', long)]
    output_path: String, // se il path non esiste lo crea
}



fn main() {
    //******************************* aggiunto per i parametri */
    //se da linea di comando inserisci -i e -o prende i valori
    //dai parametri altrimenti prende quelli dai file .txt
    //select case con 2 bracci
    let args = match Argomenti::try_parse() {
        Ok(arg) => arg,
        Err(_) => {
            let x = fs::read_to_string(FILE_ARRIVO.to_string());
            let y = fs::read_to_string(FILE_PARTENZA.to_string());
            Argomenti {
                input_path: y.unwrap(),
                output_path: x.unwrap(),
            }
        }
    }; // salva gli argomenti CLI in una nuova istanza della struct

    //***++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */


    //03 istanzio la struct
    let comprimi_file = ComprimiFile::new(&args.input_path, &args.output_path);
    match comprimi_file.esegui() {
        Ok(_) => (),
        Err(err) => println!("errore : {}", err),
    }
    println!("fine procedura di compattamento rar!");
}

//01 creo una struct per i parametri
struct ComprimiFile {
    anno_inizio: i32,
    anno_fine: i32,
    path_sorgente: String,
    path_destinazione: String,
}

//02 implentazione metodi della struttura comprimi
impl ComprimiFile {
    //metodo statico che diventa costruttore con new
    //creando una istanza di ComprimiFile
    fn new(par_path_sorgente: &str, par_path_destinazione: &str) -> ComprimiFile {
        ComprimiFile {
            anno_inizio: 1950,
            anno_fine: 2050,
            //to_owned() = significa la traduzione da &str in String
            path_sorgente: par_path_sorgente.to_owned(),
            path_destinazione: par_path_destinazione.to_owned(),
        }
    }

    //II metodo esegui &self = richiede l'istanza comprimi file
    // perche non è statico
    fn esegui(&self) -> Result<bool, Error> {
        //for partendo dagli estremi anno inizio e fine del costruttore

        let cartella = fs::read_dir(&self.path_sorgente)?;
        for file in cartella {
            match file {
                Ok(dir_entry) => {
                    //la path del file corrente
                    let file_metadata = dir_entry.metadata()?;
                    let istanza_file_time = FileTime::new(file_metadata);
                    //destrutturazione di una tupla = assegna ad anno e al mese i due
                    //valori recuperati dalla tupla istanza_file_time.get_anno_mese()
                    let (anno, mese) = istanza_file_time.get_anno_mese();
                    // TODO: aggiungere nome cartella genitore
                    let nome_file_zip = format!("{}\\{}_{}_{:#02}.rar",self.path_destinazione, get_path_parent(&dir_entry.path()), anno, mese);

                    for anno_corrente in self.anno_inizio..=self.anno_fine {
                        for mese_corrente in 1..=12 {
                            if anno == anno_corrente && mese == mese_corrente {
                                ComprimiFile::comprimi_rar(&nome_file_zip, dir_entry.path().to_str().unwrap_or(""));
                            }
                        }
                    }
                }
                Err(errore) => println!("errore di ricerca del file: {}", errore),
            }
        }
        Ok(true)
    }

    /// modello di comando multivolume per 10 mega
    /// C:\CASA\Rar.exe a -r -u ZZ_SALVATAGGI_ARCHIVI_70_GENERICI *.* -v10m
    fn comprimi_rar(par_nome_zip: &str, par_nome_file_archivio: &str) {
        //istanzio il comando rar
        let mut command = Command::new("c:\\CASA\\WinRAR\\Rar.exe");

        //predispongo i successivi parametri di rar in un vettore
        let argomenti = vec![
            "U",
            "-r",
            "-ac",
            par_nome_zip,
            par_nome_file_archivio,
            "-v1m",
        ];
        // prende l'istanza del comando a cui aggiunge gli argomenti rar
        let command = command.args(&argomenti);

        let mut s: String = String::new();
        for arg in argomenti {
            s.push_str(arg);
            s.push(' ');
        }

        println!("Rar.exe {}", s);
        //.output = eseguo il comando
        command.output().expect("failed to execute process");
    }
}


pub fn get_path_parent (path_file:&Path)->String{
    let parent =Path::parent(path_file).unwrap();
    Path::file_name(parent).unwrap().to_str().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /*esempio di testo che viene fatto partire con cargo test  e siccome
    contiene la asset_eq = effettua un vero calcolo di somma tra 2+2 e quindi il risultato
    corretto per la riuscita del testo è 4 e non 3
    Il test puo essere attivato singolarmente.
    */

    #[test]
    fn test_farlocco() {
        assert_eq!(2 + 2, 4, "Test fallito perche 2+2=4 non 3");
    }

    #[test]
    fn comprimi_rar_test() {
        ComprimiFile::comprimi_rar("prova.zip", "Cargo.toml");
        let x = Path::new("prova.zip").exists();
        assert!(x, "test fallito il file.zip non esiste");

        ComprimiFile::comprimi_rar("prova2.zip", "c:\\CASA\\PROVA_RUST\\CARTELLA_PROVA\\");
        let x = Path::new("prova2.zip.part01.rar").exists() || Path::new("prova2.zip").exists();
        assert!(x, "test fallito il file.zip non esiste");
    }
}
