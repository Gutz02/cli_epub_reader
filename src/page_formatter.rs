use std::cell::RefCell;
use std::rc::Rc;
use std::{fmt, fs::File};
use std::io::{self, BufRead, BufReader};

use colored::{ColoredString, Colorize};

use xml_reader::file_parser::parse_contents;
use xml_reader::file_reader::{file_formatter, read_xml_file};
use xml_reader::node::Node;

pub const FORMATTING_NODES  : [&str; 23 ] = [
    "p", "br", "span", "h1", "h2", "h3", "h4", "h5", "h6","strong",
    "b", "em", "i", "u", "small", "mark", "del", "ins", "sub", "sup",
    "blockquote", "q", "pre"
];

const SUPPORTED_TEXT_NODES : [&str; 11 ] = [
    "strong", "b" , "em", "i", "u", "small" , "del", "ins", "blockquote", "q",  "p"
];

const SUPPORTED_STRUCTURAL_NODES : [&str; 9 ] = [
    "br", "hr", "h1", "h2", "h3", "h4", "h5", "h6", "p"
];

impl fmt::Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {:?} || File Path: {}", self.title, self.file_path)
    }
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path
    }
}

pub enum ErrorTextFormating{
    FailedToReadChapter,
    FailedToReadLine,
    NoTitleFound
}

pub struct Chapter {
    title : Option<String>,
    content : String,
    file_path : String,
    all_nodes : Option<Vec<Rc<Node>>>
}


impl Chapter {

    pub fn new(file_path : String) -> Self{
        Chapter{
            title : None,
            content : String::new(),
            file_path : file_path,
            all_nodes : None
        }
    }

    pub fn get_title(&self) -> Option<String>{
        self.title.clone()
    }

    pub fn get_file_path(&self) -> String{
        self.file_path.clone()
    }

    pub fn get_all_nodes(&self) -> Option<&Vec<Rc<Node>>>{
        match &self.all_nodes {
            Some(all_nodes) => Some(&all_nodes),
            None => None
        }
    }

    pub fn start_chapter(&mut self)-> Result<(), ErrorTextFormating>{
        let contents = read_xml_file(self.file_path.as_str());
        match contents{
            Ok(data) => {
                let formatted_contents: Vec<String> = file_formatter(&data);
                self.all_nodes = Some(parse_contents(&formatted_contents));
                self.create_page();
                println!("{}",self.content);
                return Ok(())
            },
            Err(_) => {
                println!("Failed to read chapter at {}",self.file_path);
                return Err(ErrorTextFormating::FailedToReadChapter);
            }
        }
    }

    pub fn get_content_nodes(all_nodes : &Vec<Rc<Node>>) -> Vec<usize>{
        let mut ordered_contents_id = vec![];

        for node in all_nodes{
            if Chapter::is_formating_node(node.get_name()){
                ordered_contents_id.push(node.get_id());
            }
        }

        ordered_contents_id
    }


    pub fn create_page(&mut self){

        if let Some(all_nodes) = &self.all_nodes  {
            let head_node = all_nodes[0].clone();
            self.apply_format_(&head_node);
        }

    }

    pub fn apply_format_(&mut self, node : &Rc<Node>){

        for child in &node.get_children().clone(){
            if child.get_name() == "content"{
                self.apply_text(node);
            }
            self.apply_format_(child);
        }
        self.apply_structural(node);
    }

    fn apply_text(&mut self, node : &Rc<Node>){
        if SUPPORTED_TEXT_NODES.contains(&node.get_name().as_str()){
            self.content = format!("{}{}",self.content, Chapter::apply_text_node(node.get_contents().clone(), node.get_name()));
        }
    }

    fn apply_structural(&mut self, node : &Rc<Node>){
        if SUPPORTED_STRUCTURAL_NODES.contains(&node.get_name().as_str()) {
            self.content = format!("{}{}",self.content, Chapter::apply_structural_node(node.get_contents().clone(), node.get_name()));
        }
    }

    fn apply_text_node(text: Option<String>, node: &str) -> colored::ColoredString {
        if let Some(text) = text {
            match node {
                "strong" | "b" => text.bold(),
                "em" | "i"     => text.italic(),
                "u" | "ins"    => text.underline(),
                "del"          => text.strikethrough(),
                "blockquote" | "q" => format!("“{text}”").normal(),
                "p" => text.normal(),
                _ => String::new().normal(),
            }
        } else {
            String::new().normal()
        }
    }
    

    fn apply_structural_node(text: Option<String>, node: &str) -> colored::ColoredString{
        if let Some(text) = text{
            match node {
                "h1" => format!("\t\t\tHeading 1: {}\n",text).normal(),
                "h2" => format!("\t\tHeading 2: {}\n",text).normal(),
                "h3" => format!("\tHeading 3: {}\n",text).normal(),
                "p"  => "\n\n".normal(),
                _ => String::new().normal(), 
            }
        } else {
            match node {
                "br" => "\n".to_string().normal(),
                "p"  => "\n\n".normal(),
                _ => String::new().normal()
            }
        }
    }

    fn newline(&mut self, node : &Rc<Node>){
        if node.get_name() == "p"{
            self.content = format!("{}{}",self.content, "\n\n".to_string())
        }
    }


    fn is_formating_node(node_name : &String) -> bool{
        FORMATTING_NODES.contains(&(node_name.as_str()))
    }


}