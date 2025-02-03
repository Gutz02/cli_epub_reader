use std::{collections::HashMap, fs, iter::Map};
use cli_epub_reader::epub_to_components::*;
use xml_reader::file_parser::parse_contents;

#[test]
fn test_file_not_found(){
    let title = "kate-chopin_short.epub".to_string();
    let error = read_epub(&title);
    assert_eq!(error, Err(MyError::FileNotFound));
}

#[test]
fn test_epub_folder_created(){
    delete_directory( "src\\processed_files\\kate-chopin_short-fiction");

    let title = "kate-chopin_short-fiction".to_string();
    read_epub(&title);
    assert!(folder_exists(&"src\\processed_files\\kate-chopin_short-fiction".to_string()));

}

#[test]
fn test_epub_folder_already_exist(){
    delete_directory( "src\\processed_files\\kate-chopin_short-fiction");
    let folder_name = "kate-chopin_short-fiction".to_string();
    read_epub(&folder_name);
    let error = read_epub(&folder_name);
    assert_eq!(error, Err(MyError::FolderAlreadyExists));
}

#[test]
fn test_zip_file_created(){
    delete_directory( "src\\processed_files\\kate-chopin_short-fiction");

    let folder_name = "src\\new_files\\kate-chopin_short-fiction.epub".to_string();
    let zip_location = "src\\processed_files\\kate-chopin_short-fiction".to_string();
    let some_err = unzip_epub(&folder_name, &zip_location);
    let file_count = fs::read_dir(zip_location)
    .expect("Failed to read directory")
    .count();
    assert_eq!(file_count, 3, "The number of extracted files is not 3");

}

#[test]
fn test_read_content_xml(){
    delete_directory( "src\\processed_files\\kate-chopin_short-fiction");

    let folder_name = "src\\new_files\\kate-chopin_short-fiction.epub".to_string();
    let zip_location = "src\\processed_files\\kate-chopin_short-fiction".to_string();
    unzip_epub(&folder_name, &zip_location);

    let zip_location = "src\\processed_files\\kate-chopin_short-fiction".to_string();
    let rootfile_path : String = find_container(&zip_location).unwrap();
    assert_eq!(rootfile_path, "epub/content.opf");

}

#[test]
fn test_reading_order(){
    let spine: Vec<String> = vec![
        "<spine>".to_string(),
        "   <itemref idref=\"titlepage.xhtml\"/>".to_string(),
        "   <itemref idref=\"imprint.xhtml\"/>".to_string(),
        "   <itemref idref=\"with-the-violin.xhtml\"/>".to_string(),
        "</spine>".to_string()
    ];

    let manifest : Vec<String> = vec![
        "<manifest>".to_string(),
        "   <item href=\"text/with-the-violin.xhtml\" id=\"with-the-violin.xhtml\" media-type=\"application/xhtml+xml\"/>".to_string(),
        "   <item href=\"text/titlepage.xhtml\" id=\"titlepage.xhtml\" media-type=\"application/xhtml+xml\"/>".to_string(),
        "   <item href=\"text/imprint.xhtml\" id=\"imprint.xhtml\" media-type=\"application/xhtml+xml\"/>".to_string(),
        "</manifest>".to_string()
    ];

    let spine_nodes = parse_contents(&spine);
    let manifest_nodes = parse_contents(&manifest);

    let reading_list = get_reading_order(&spine_nodes, &manifest_nodes);

    assert_eq!(reading_list[0], 2);
    assert_eq!(reading_list[1], 3);
    assert_eq!(reading_list[2], 1);

}

#[test]
fn test_table_of_contents(){
    let content_opf: Vec<String> = vec![
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>".to_string(),
        "<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"uid\">".to_string(),
    
        "  <manifest>".to_string(),
        "    <item href=\"text/with-the-violin.xhtml\" id=\"with-the-violin.xhtml\" media-type=\"application/xhtml+xml\"/>".to_string(),
        "    <item href=\"text/titlepage.xhtml\" id=\"titlepage.xhtml\" media-type=\"application/xhtml+xml\"/>".to_string(),
        "    <item href=\"text/imprint.xhtml\" id=\"imprint.xhtml\" media-type=\"application/xhtml+xml\"/>".to_string(),
        "  </manifest>".to_string(),
    
        "  <spine toc=\"ncx\">".to_string(),
        "    <itemref idref=\"titlepage.xhtml\"/>".to_string(),
        "    <itemref idref=\"imprint.xhtml\"/>".to_string(),
        "    <itemref idref=\"with-the-violin.xhtml\"/>".to_string(),
        "  </spine>".to_string(),
    
        "</package>".to_string(),
    ];

    let all_nodes = parse_contents(&content_opf);
    let idref_href : Vec<usize> = spine_to_manifest(&all_nodes);
    format_table_contents(idref_href, &all_nodes);
}
