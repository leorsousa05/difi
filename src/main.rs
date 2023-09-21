use std::{fs::read_dir, os::windows::prelude::MetadataExt};
use std::path::Path;
use std::env::args;
use chrono::{DateTime, Utc};
use anyhow::Result;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::{Style, Disable};
use tabled::settings::locator::ByColumnName;

#[derive(Tabled)]
struct FilesAndDirsInfos {
    filename: String,
    created: String,
    last_modified: String,
    size: String
}

fn main() {
    let args: Vec<String> = args().collect();
    let options: Vec<String> = args.clone().into_iter().filter(|arg| arg.contains(&"-")).collect();

    if args.get(1).is_none() {
        let _ = read_directory(Path::new("."), options);
    } else {
        let path_exists = Path::new(&args[1]).try_exists().unwrap();

        if path_exists {
            let _ = read_directory(Path::new(&args[1]), options);
        } else {
            let _ = read_directory(Path::new("."), options);
        }
    }

}

fn read_directory(path: &Path, options: Vec<String>) -> Result<()> {

    let path_is_dir = path.is_dir();
    let path_is_file = path.is_file();
    let path_exists = !path.try_exists()?;

    if path_is_dir {
        let mut formatted_dirs: Vec<FilesAndDirsInfos> = Vec::new(); 

        for entry in read_dir(path)? {
            let entry = entry?;
            let elapsed_created_time_files = entry.metadata()?.created()?;
            let elapsed_modified_time_files = entry.metadata()?.modified()?;
            let file_size = entry.metadata()?.file_size().to_string();

            let created_datetime: DateTime<Utc> = elapsed_created_time_files.into();
            let modified_datetime: DateTime<Utc> = elapsed_modified_time_files.into();

            let is_dir = entry.path().is_dir();
            let mut file: String = entry.file_name().to_str().unwrap().to_string();

            if is_dir {
                file = file + "/";
            }

            formatted_dirs.push(FilesAndDirsInfos {
                filename: file,
                created: created_datetime.format("%d/%m/%Y").to_string(),
                last_modified: modified_datetime.format("%d/%m/%Y").to_string(),
                size: file_size
            });
        }

        let mut table = Table::new(formatted_dirs);
        table
            .with(Style::blank());

        if options.is_empty() {
            table
                .with(Disable::column(ByColumnName::new("created")))
                .with(Disable::column(ByColumnName::new("last_modified")))
                .with(Disable::column(ByColumnName::new("size")));
        }

        for option in &options[0..] {
            if !option.eq("-A") {
                table
                    .with(Disable::column(ByColumnName::new("created")))
                    .with(Disable::column(ByColumnName::new("last_modified")))
                    .with(Disable::column(ByColumnName::new("size")));
            }
        }

        println!("{}", table);
    }

    if path_is_file {
        eprintln!("Path is a File, send a Directory!");
    }

    if path_exists {
        eprintln!("Path does not exist!");
    }


    Ok(())
}
