// Sortownik do plikow
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    //uzytkownik wpisuje sciezke do folderu
    println!("Podaj ścieżkę folderu z plikami:");
    let mut folder_path = String::new();
    io::stdin().read_line(&mut folder_path)?;
    let folder_path = folder_path.trim();

    //uzyrkownik wpisuje sciezke docelowa
    println!("Podaj ścieżkę docelową dla posortowanych plików:");
    let mut target_path = String::new();
    io::stdin().read_line(&mut target_path)?;
    let target_path = target_path.trim();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_size = metadata.len();

       
        let target_folder = if file_size <= 5_000_000 {
            "1-5MB"
        } else if file_size <= 10_000_000 {
            "5-10MB"
        } else {
            "10MB+"
        };

        //tworymy folder jak go jeszcze nie ma
        let target_folder_path = Path::new(target_path).join(target_folder);
        fs::create_dir_all(&target_folder_path)?;

        //przenosimy plik do wyznaczonego foldreu
        let file_name = entry.file_name();
        let target_file_path = target_folder_path.join(file_name);
        fs::rename(entry.path(), target_file_path)?;
    }

    println!("Pliki zostały posortowane.");
    Ok(())
}
