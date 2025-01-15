use std::{fs, io::{self, BufReader}, path::Path, rc::Rc};

use xml_reader::{file_parser::parse_contents, file_reader::read_xml_file, node::Node};
use zip::ZipArchive;

#[derive(Debug, PartialEq)]
pub enum MyError{
    FileNotFound,
    FailedToCreateFolder,
    FolderAlreadyExists,
    FailedToCreateZip,
    FailedToRead,
    NotEpub,
    FailedReadingOrder
}

const PROCESSED_FOLDER_DIRECTORY : &str = "src\\processed_files";
const UNPROCESSED_FOLDER_DIRECTORY : &str = "src\\new_files";

pub fn read_epub(title: &String) -> Result<String, MyError> {
    let file_location = format!("{UNPROCESSED_FOLDER_DIRECTORY}\\{title}.epub");
    if !Path::new(&file_location).is_file(){
        return Err(MyError::FileNotFound);
    }

    let file_dir = format!("{PROCESSED_FOLDER_DIRECTORY}\\{title}");
    if folder_exists(&file_dir) {
        return Err(MyError::FolderAlreadyExists);
    }

    create_folder(&file_dir).map_err(|e| MyError::FailedToCreateFolder)?;
    Ok(file_dir)
}

pub fn unzip_epub(epub_path : &String, new_directory: &String) -> io::Result<()>{

    let extract_dir = new_directory;
    let file = fs::File::open(epub_path).unwrap();
    let reader = BufReader::new(file);
    let mut zip = ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let out_path = std::path::Path::new(extract_dir).join(file.name());

        if file.is_dir() {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // println!("Extracted: {}", out_path.display());
    }

    println!("EPUB extraction complete!");
    Ok(())
}


pub fn find_container(root_path : &String) -> Result<String, MyError>{
    let meta_path = format!("{root_path}\\META-INF\\container.xml");
    let meta_string : Result<String, String> = read_xml_file(&meta_path);
    match meta_string {
        Ok(data) => get_container_location(&data),
        Err(_) => Err(MyError::FailedToRead)
    }
}

pub fn get_container_location(file_lines : &String) -> Result<String, MyError>{
    let list_lines: Vec<String> = file_lines.lines().map(|line| line.to_string()).collect();
    let all_nodes = parse_contents(&list_lines);

    let mut iter_rootfile = all_nodes.iter().filter( |node| node.get_name() == "rootfile");
    match iter_rootfile.next() {
        Some(node) => return Ok(node.get_attribute("full-path".to_string()).unwrap().clone()),
        None => Err(MyError::FileNotFound)
    }
}

pub fn spine_to_manifest(all_nodes: &Vec<Rc<Node>>) -> Vec<usize> {
    let spine_node = all_nodes.iter().find(|node| node.get_name() == "spine").unwrap();
    let manifest_node = all_nodes.iter().find(|node| node.get_name() == "manifest").unwrap();

    let spine_children = spine_node.get_children().clone();
    let manifest_children = manifest_node.get_children().clone();


    get_reading_order(&spine_children, &manifest_children)
}


pub fn get_reading_order(spine_nodes: &Vec<Rc<Node>>, manifest_nodes: &Vec<Rc<Node>>) -> Vec<usize> {
    let mut reading_order: Vec<usize> = Vec::new();

    for node in spine_nodes {
        if let Some(reference_id) = node.get_attribute("idref".to_string()) {
            let twin_node = manifest_nodes.iter().find(|node| {
                node.get_attribute("id".to_string()).unwrap_or(&"".to_string()) == reference_id
            });

            match twin_node {
                Some(node) => reading_order.push(node.get_id()),
                None => {},
            }
        }
    }
    reading_order
}

pub fn format_table_contents(ordered_files : Vec<usize>, all_nodes: &Vec<Rc<Node>>) -> String {
    let mut table_contents = "===============Table Of Contents===============\n".to_string();


    for id in ordered_files{
        let current_node = all_nodes.iter().find(
            |node| node.get_id() == id 
        ).unwrap(); // 100% this node exists 

        let title = current_node.get_attribute(String::from("id")).unwrap()
            .replace("-"," ")
            .replace(".xhtml", "");
        table_contents.push_str(
            &format!("{title}\n")
        );
        println!("{title}")
    }

    println!("{table_contents}");

    table_contents
}



pub fn folder_exists(path: &String) -> bool {
    Path::new(path).is_dir()
}

pub fn create_folder(path: &String) -> Result<(), MyError> {
    match fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(MyError::FailedToCreateFolder),
    }
}

pub fn delete_directory(path: &str) -> Result<(), std::io::Error> {
    let folder_path = Path::new(path);

    if folder_path.exists() && folder_path.is_dir() {
        fs::remove_dir_all(folder_path)?;
        println!("Folder '{}' deleted successfully.", path);
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Folder '{}' not found or is not a directory.", path),
        ))
    }
}