use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let folder_path = get_folder_path_from_user()?;
    let folder_name = Path::new(&folder_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let action = ask_user_action()?;
    let traverse_subfolders = ask_traverse_subfolders()?;

    // Traverse the folder and process files
    process_directory(&folder_path, folder_name, &action, traverse_subfolders)?;

    Ok(())
}

fn get_folder_path_from_user() -> io::Result<PathBuf> {
    print!("Please enter the full path of the folder: ");
    io::stdout().flush()?; // Ensure the prompt is printed before user input

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim(); // Trim any trailing newline
    let path = PathBuf::from(input);

    if !path.is_dir() {
        eprintln!("The provided path is not a directory or doesn't exist.");
        std::process::exit(1);
    }

    Ok(path)
}

fn ask_user_action() -> io::Result<String> {
    print!("Do you want to (add) the folder name as a prefix or (remove) it? (add/remove): ");
    io::stdout().flush()?; // Ensure the prompt is printed before user input

    let mut action = String::new();
    io::stdin().read_line(&mut action)?;

    let action = action.trim().to_lowercase(); // Normalize input to lowercase and trim any whitespace

    if action != "add" && action != "remove" {
        eprintln!("Invalid action. Please type 'add' or 'remove'.");
        std::process::exit(1);
    }

    Ok(action)
}

fn ask_traverse_subfolders() -> io::Result<bool> {
    print!("Do you want to traverse subfolders? (yes/no): ");
    io::stdout().flush()?; // Ensure the prompt is printed before user input

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;

    let answer = answer.trim().to_lowercase(); // Normalize input to lowercase and trim any whitespace

    match answer.as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => {
            eprintln!("Invalid response. Please type 'yes' or 'no'.");
            std::process::exit(1);
        }
    }
}

fn process_directory(folder_path: &Path, current_folder_name: &str, action: &str, traverse_subfolders: bool) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Process file based on the chosen action
            match action {
                "add" => add_prefix(&entry, current_folder_name)?,
                "remove" => remove_prefix(&entry, current_folder_name)?,
                _ => eprintln!("Invalid action selected."),
            }
        } else if traverse_subfolders && path.is_dir() {
            // Extract the subfolder name
            let subfolder_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Combine current folder and subfolder name for the new prefix
            let new_prefix = subfolder_name;

            // Recursively traverse subfolders with updated prefix
            process_directory(&path, &new_prefix, action, traverse_subfolders)?;
        }
    }
    Ok(())
}

fn add_prefix(entry: &DirEntry, folder_name: &str) -> io::Result<()> {
    let file_name = entry.file_name();
    let file_name_str = file_name.to_str().unwrap();

    // Check if the file is already prefixed
    if !file_name_str.starts_with(folder_name) {
        let new_name = format!("{}_{}", folder_name, file_name_str);
        let new_path = entry.path().with_file_name(new_name);

        // Rename the file
        fs::rename(entry.path(), new_path.clone())?;
        println!("Renamed: {} -> {}", file_name_str, new_path.display());
    } else {
        println!("Already prefixed: {}", file_name_str);
    }

    Ok(())
}

fn remove_prefix(entry: &DirEntry, folder_name: &str) -> io::Result<()> {
    let file_name = entry.file_name();
    let file_name_str = file_name.to_str().unwrap();

    // Check if the file has the folder name prefix
    if file_name_str.starts_with(folder_name) {
        let new_name = file_name_str.trim_start_matches(&format!("{}_", folder_name));
        let new_path = entry.path().with_file_name(new_name);

        // Rename the file
        fs::rename(entry.path(), new_path.clone())?;
        println!("Renamed: {} -> {}", file_name_str, new_path.display());
    } else {
        println!("No prefix to remove: {}", file_name_str);
    }

    Ok(())
}
