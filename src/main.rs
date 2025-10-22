use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use std::fs::File;
use std::process::Command;

//--------------------compiler--------------------

pub trait Compiler {
/// Begin the compilation process (entry point).
fn compile(&mut self, source: &str);
/// Get the next token from the lexical analyzer.
fn next_token(&mut self) -> String;
/// Run the syntax analyzer starting from <lolcode>.
fn parse(&mut self);
/// Get the current token being processed.
fn current_token(&self) -> String;
/// Set the current token (typically used internally).
fn set_current_token(&mut self, tok: String);
}

//struct that basically runs everything, holds lexer and syntaxer (and technically semantic analysis too)
pub struct LolcodeCompiler{
    lexer: LolcodeLexicalAnalyzer,
    current_token: String,
    syntaxer: LolcodeSyntaxAnalyzer,
}

impl LolcodeCompiler{
    //prepares everything to run properly
    pub fn new() -> Self{
        Self {
            lexer: LolcodeLexicalAnalyzer::new(""),
            current_token: String::new(),
            syntaxer: LolcodeSyntaxAnalyzer::new()
        }
    }

    }

impl Compiler for LolcodeCompiler{
    //method opens a new lexer, then it begins tokenization which runs the entire string through it and puts tokens into the tokens vector
    fn compile(&mut self, source: &str){
        self.lexer = LolcodeLexicalAnalyzer::new(source);
        self.lexer.tokenize();
        self.parse();
    }
    //pops token off vector and returns whatever it is, also has error detection functionality which is somewhat redundant since its also built into the tokenizer method
    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if self.lexer.lookup(&candidate) || !candidate.starts_with('#') {
            self.current_token = candidate.clone();
            candidate
        } else if self.lexer.tokens.is_empty() {
            self.current_token.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }
    //basically just repeatedly calls next token to get tokens into the syntax analyzer, also completely disregards empty tokens to make syntax analysis go smoother
    //is it an "interesting" use of the next token function, yes, does it work though, also yes
    fn parse(&mut self){
        while !self.lexer.tokens.is_empty() {
        let mut tok = self.next_token();
        if tok.is_empty() {
            break;
        }
        if tok.trim().is_empty() {
            continue;
        }
        self.syntaxer.token_vector.push(std::mem::take(&mut tok));
        }
    self.syntaxer.parse_lolcode();
    }

    fn current_token(&self) -> String{
        self.current_token.clone()
    }

    fn set_current_token(&mut self, token: String){
        self.current_token = token;
    }
}

//--------------------lexical analyzer--------------------

pub trait LexicalAnalyzer {
/// Return the next character from the input.
/// If input is exhausted, should terminate the program.
fn get_char(&mut self) -> char;
/// Add a character to the current potential token.
fn add_char(&mut self, c: char);
/// Lookup a potential token to determine if it is valid.
/// Returns true if a valid token/lexeme, false otherwise.
fn lookup(&self, s: &str) -> bool;
}

//struct that stores everything related to the lexical analyzer, including the afformentioned tokens vector and all of the valid tokens in lolcode
pub struct LolcodeLexicalAnalyzer{
    input: Vec<char>,
    position: usize,
    current_build: String,
    pub tokens: Vec<String>,
    //i changed all of the variable names to ..._... because i was getting very annoying warnings about them being named in the "somethingSomething" convention
    pub lolcode_begin : String,
    pub lolcode_end : String,
    pub comment_begin : String,
    pub comment_end : String,
    pub head_begin : String,
    pub end_one : String,
    pub title_begin : String,
    pub end_two : String,
    pub paragraph_begin : String,
    pub bold_begin : String,
    pub italics_begin : String,
    pub list_begin : String,
    pub list_item_begin : String,
    pub newline : String,
    pub audio_begin : String,
    pub video_begin : String,
    pub variable_begin : String,
    pub variable_middle : String,
    pub variable_use : String,
}

impl LolcodeLexicalAnalyzer{
    pub fn new(source: &str) -> Self{
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            lolcode_begin : "#HAI".into(),
            lolcode_end : "#KTHXBYE".into(),
            comment_begin : "#OBTW".into(),
            comment_end : "#TLDR".into(),
            head_begin : "#MAEK HEAD".into(),
            end_one : "#OIC".into(),
            title_begin : "#GIMMEH TITLE".into(),
            end_two : "#MKAY".into(),
            paragraph_begin : "#MAEK PARAGRAF".into(),
            bold_begin : "#GIMMEH BOLD".into(),
            italics_begin : "#GIMMEH ITALICS".into(),
            list_begin : "#MAEK LIST".into(),
            list_item_begin : "#GIMMEH ITEM".into(),
            newline : "#GIMMEH NEWLINE".into(),
            audio_begin : "#GIMMEH SOUNDZ".into(),
            video_begin : "#GIMMEH VIDZ".into(),
            variable_begin : "#I HAZ".into(),
            variable_middle : "#IT IZ".into(),
            variable_use : "#LEMME SEE".into(),
        }
    }
    //this is by far the most complicated function in the entire project, so ill try to describe what its doing as well as possible
    //in simple terms, it tracks if the current token is text or a tag and handles the tokens differently based on which it is
    //if its text, no lookup is needed and it just throws it into the tokens vector
    //if its a tag though, the tag is set to uppercase to make lookup easier, then theres functionality that basically allows the tokenizer to look ahead to see if the token is 2 words or not
    //if the tag token is a valid one word token, cool, throws it into the vector
    //if its not found to be a valid one word token, it then grabs the next word and performs lookup again
    //if lookup comes back good, cool, throw it into the vector, if not, the program exits immediately since an invalid token was found
    //TLDR, this method both tokenizes input and performs lexical analysis on the tokens in one fell swoop
    pub fn tokenize(&mut self) {
    let mut in_hash_token = false;

    loop {
        let c = self.get_char();

        if c == '\0' {
            if !self.current_build.is_empty() {
                // finalize last token
                if in_hash_token {
                    // uppercase all hashtag tokens
                    let token = self.current_build.trim_end().to_uppercase();
                    self.tokens.push(token);
                } else {
                    self.tokens.push(std::mem::take(&mut self.current_build));
                }
            }
            break;
        }

        if c == '#' {
            if !self.current_build.is_empty() && !in_hash_token {
                self.tokens.push(std::mem::take(&mut self.current_build));
            }

            in_hash_token = true;
            self.current_build.push(c);
        } 
        else if c.is_whitespace() {
    if in_hash_token {
        let mut lookahead_pos = self.position;
        let mut next_word = String::new();
        while lookahead_pos < self.input.len() && self.input[lookahead_pos].is_whitespace() {
            lookahead_pos += 1;
        }
        while lookahead_pos < self.input.len() && !self.input[lookahead_pos].is_whitespace() {
            next_word.push(self.input[lookahead_pos]);
            lookahead_pos += 1;
        }
        let single = self.current_build.trim_end().to_uppercase();
        let combined = format!("{} {}", self.current_build.trim_end(), next_word).to_uppercase();

        let mut final_token = single.clone();
        if self.lookup(&combined) {
            for _ in 0..next_word.len() {
                self.get_char();
            }
            final_token = combined;
        } else if !self.lookup(&single) {
            if combined.starts_with('#') {
                final_token = combined;
            }
        }
        self.tokens.push(final_token.clone());
        if !self.lookup(&final_token) {
            eprintln!("lexical error: '{}' is not a recognized token", final_token);
            std::process::exit(1);
        }

        self.current_build.clear();
        in_hash_token = false;
    } else {
        self.add_char(c);
    }
}
        else {
            self.add_char(c);
        }
    }

    self.tokens.reverse();
}
}

impl LexicalAnalyzer for LolcodeLexicalAnalyzer{
    //this and the add char method basically just grab the next character in the input, then adds the character to the current build
    fn get_char(&mut self) -> char{
        if self.position < self.input.len(){
            let c = self.input[self.position];
            self.position += 1;
            c
        } else{
            '\0'
        }
    }

    fn add_char(&mut self, c: char){
        self.current_build.push(c);
    }

    fn lookup(&self, s: &str) -> bool{
        if !s.starts_with('#'){
            return false;
        }
        self.lolcode_begin == s
        || self.lolcode_end == s
        || self.comment_begin == s
        || self.comment_end == s
        || self.head_begin == s
        || self.end_one == s
        || self.title_begin == s
        || self.end_two == s
        || self.paragraph_begin == s
        || self.bold_begin == s
        || self.italics_begin == s
        || self.list_begin == s
        || self.list_item_begin == s
        || self.newline == s
        || self.audio_begin == s
        || self.video_begin == s
        || self.variable_begin == s
        || self.variable_middle == s
        || self.variable_use == s
    }
}

//--------------------syntax analyzer--------------------

pub trait SyntaxAnalyzer {
fn grab_token(&mut self, token: String);
fn parse_lolcode(&mut self);
fn parse_head(&mut self);
fn parse_title(&mut self);
fn parse_comment(&mut self);
fn parse_body(&mut self);
fn parse_paragraph(&mut self);
fn parse_inner_paragraph(&mut self);
fn parse_inner_text(&mut self);
fn parse_variable_define(&mut self);
fn parse_variable_use(&mut self);
fn parse_bold(&mut self);
fn parse_italics(&mut self);
fn parse_list(&mut self);
fn parse_list_items(&mut self);
fn parse_inner_list(&mut self);
fn parse_audio(&mut self);
fn parse_video(&mut self);
fn parse_newline(&mut self);
fn parse_text(&mut self);
fn parse_tree_push(&mut self);
fn next_token(&mut self);
}

pub struct LolcodeSyntaxAnalyzer{
    pub token_vector: Vec<String>,
    pub parse_tree : Vec<String>,
    pub current_token : String,
    pub output : String,
}

impl LolcodeSyntaxAnalyzer{
    pub fn new() -> Self{
        Self {
            token_vector: Vec::new(),
            parse_tree: Vec::new(),
            current_token : String::new(),
            output: String::new(),
        }
    }
}

impl SyntaxAnalyzer for LolcodeSyntaxAnalyzer{
    
    //this is the function that grabs the tokens from the parse method in lexical analysis, it then stores the tokens to a different token vector for usage in syntax analysis
    fn grab_token(&mut self, token: String){
        self.token_vector.push(token);
    }
    //method to push tokens to the parse tree quickly (basically made it so that i didnt have to write out this line way more)
    fn parse_tree_push(&mut self){
        self.parse_tree.push(self.current_token.clone());
    }
    //method that quickly grabs the next token from the token vector (once again made it so i didnt have to write out that line a bunch)
    fn next_token(&mut self){
        self.current_token = self.token_vector.pop().unwrap();
    }
    //this function is basically the driver for syntax analysis, this is where it both starts and ends
    //after everything is analyzed (assuming it makes it through and is found to be valid), the parse tree is sent to semantic analysis and it begins semantic analysis
    //this method also grabs the output html from semantic analysis as well so that it was easier to access from main
    //regarding the specifics of syntax analysis, most of it is pretty straightforward but i did write some commentary for the more interesting parts of it
    fn parse_lolcode(&mut self){
        self.token_vector.reverse();
        self.next_token();
        if self.current_token != "#HAI" {
            eprintln!("syntax error: expected #HAI but found {} instead", self.current_token);
            std::process::exit(1);
        }
        self.parse_tree_push();
        self.next_token();
        self.parse_comment();
        self.parse_head();
        self.parse_body();
        if self.current_token != "#KTHXBYE" {
            eprintln!("syntax error: expected #KTHXBYE but found {} instead", self.current_token);
            std::process::exit(1);
        }
        self.parse_tree_push();
        if !self.token_vector.is_empty() {
            eprintln!("syntax error: extra tokens found after #KTHXBYE");
            std::process::exit(1);
        }
        let mut semantics = LolcodeSemanticAnalyzer::new();
        semantics.parse_tree = self.parse_tree.clone();
        semantics.semantic_analysis();
        self.output = semantics.output.clone();
    }
    
    //this is basically just an extended call of parse_title since its the only place where the title can appear
    fn parse_head(&mut self){
        if self.current_token == "#MAEK HEAD" {
            self.parse_tree_push();
            self.next_token();
            self.parse_title();
            if self.current_token == "#OIC" {
                self.parse_tree.push("#HEAD END".to_string());
                self.next_token();
                return;
            } else {
                eprintln!("syntax error: expected #OIC but found {} instead", self.current_token);
                std::process::exit(1);
            }
        } else{
            return;
        }
    }
    fn parse_title(&mut self){
        if self.current_token == "#GIMMEH TITLE" {
            self.parse_tree_push();
            self.next_token();
            if !self.current_token.starts_with("#") {
                self.parse_text();
                if self.current_token == "#MKAY" {
                    self.parse_tree.push("#TITLE END".to_string());
                    self.next_token();
                    return;
                } else {
                    eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                    std::process::exit(1);
                }
            } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
            }
        } else {
            eprintln!("syntax error: expected #GIMMEH TITLE but found {} instead", self.current_token);
            std::process::exit(1);
        }
    }
    //since comments can be repeated over and over again before the body, the method calls itself upon completion, but if #OBTW isnt found again, it just returns anyways
    fn parse_comment(&mut self){
        if self.current_token == "#OBTW" {
            self.parse_tree_push();
            self.next_token();
            if !self.current_token.starts_with("#") {
                self.parse_text();
                if self.current_token == "#TLDR" {
                    self.parse_tree_push();
                    self.next_token();
                    self.parse_comment();
                } else {
                    eprintln!("syntax error: expected #TLDR but found {} instead", self.current_token);
                    std::process::exit(1);
                }
            } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
            }

        } else {
            return;
        }
    }
    //basically just keeps looking for tokens until #KTHXBYE is found since its always the last token in the body
    fn parse_body(&mut self){
        if self.current_token == "#KTHXBYE" {
            return
        } else if !self.current_token.starts_with("#") {
            self.parse_text();
            self.parse_body();
        } else if self.current_token == "#GIMMEH BOLD" {
            self.parse_bold();
            self.parse_body();
        } else if self.current_token == "#GIMMEH ITALICS" {
            self.parse_italics();
            self.parse_body();
        } else if self.current_token == "#GIMMEH NEWLINE" {
            self.parse_newline();
            self.parse_body();
        } else if self.current_token == "#MAEK LIST" {
            self.parse_list();
            self.parse_body();
        } else if self.current_token == "#GIMMEH NEWLINE" {
            self.parse_newline();
            self.parse_body();
        } else if self.current_token == "#GIMMEH SOUNDZ" {
            self.parse_audio();
            self.parse_body();
        } else if self.current_token == "#GIMMEH VIDZ" {
            self.parse_video();
            self.parse_body();
        } else if self.current_token == "#I HAZ" {
            self.parse_variable_define();
            self.parse_body();
        } else if self.current_token == "#LEMME SEE" {
            self.parse_variable_use();
            self.parse_body();
        } else if self.current_token == "#MAEK PARAGRAF" {
                self.parse_paragraph();
                self.parse_body();
        } else if self.current_token == "#OBTW" {
            self.parse_comment();
            self.parse_body();
        } else {
                eprintln!("syntax error: expected valid body token but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    fn parse_paragraph(&mut self){
            self.parse_tree_push();
            self.next_token();
            self.parse_variable_define();
            self.parse_inner_paragraph();
            return;
    }
    fn parse_inner_paragraph(&mut self){
        if self.current_token == "#OIC" {
            self.parse_tree.push("#PARAGRAPH END".to_string());
            self.next_token();
            return;
        }
        else {
            self.parse_inner_text();
        }
    }
    //basically the same logic as the body parsing
    fn parse_inner_text(&mut self){
        if !self.current_token.starts_with("#") {
            self.parse_text();
            self.parse_inner_paragraph();
        } else if self.current_token == "#LEMME SEE" {
            self.parse_variable_use();
            self.parse_inner_paragraph();
        } else if self.current_token == "#GIMMEH BOLD" {
            self.parse_bold();
            self.parse_inner_paragraph();
        } else if self.current_token == "#GIMMEH ITALICS" {
            self.parse_italics();
            self.parse_inner_paragraph();
        } else if self.current_token == "#GIMMEH NEWLINE" {
            self.parse_newline();
            self.parse_inner_paragraph();
        } else if self.current_token == "#GIMMEH SOUNDZ" {
            self.parse_audio();
            self.parse_inner_paragraph();
        } else if self.current_token == "#GIMMEH VIDZ" {
            self.parse_video();
            self.parse_inner_paragraph();
        } else if self.current_token == "#MAEK LIST" {
            self.parse_list();
            self.parse_inner_paragraph();
        } else {
                eprintln!("syntax error: expected valid paragraph body token but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    //relatively simple method besides when it checks to see if the variable name is valid, it basically checks to see if theres any spaces in it, and if there is, gives an error
    fn parse_variable_define(&mut self){
        if self.current_token == "#I HAZ" {
            self.parse_tree_push();
            self.next_token();
            if !self.current_token.starts_with("#") {
                let var_test = self.current_token.trim();
                if var_test.contains(' ') {
                            eprintln!("syntax error: {} is an invalid variable name", self.current_token);
                            std::process::exit(1);
                }
                    self.parse_tree.push(var_test.to_string());
                    self.next_token();
                if self.current_token == "#IT IZ" {
                    self.parse_tree_push();
                    self.next_token();
                    if !self.current_token.starts_with("#") {
                        self.parse_text();
                        if self.current_token == "#MKAY" {
                            self.parse_tree.push("#VARIABLE INIT END".to_string());
                            self.next_token();
                            return;
                        } else{
                            eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("syntax error: expected text but found {} instead", self.current_token);
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("syntax error: expected #IT IZ but found {} instead", self.current_token);
                    std::process::exit(1);
                } 
            } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
            }
        } else {
            return;
        }
    }
    fn parse_variable_use(&mut self){
        self.parse_tree_push();
        self.next_token();
        if !self.current_token.starts_with("#") {
                let var_test = self.current_token.trim();
                if var_test.contains(' ') {
                            eprintln!("syntax error: {} is an invalid variable name", self.current_token);
                            std::process::exit(1);
                }
                    self.parse_tree.push(var_test.to_string());
                    self.next_token();
                if self.current_token == "#MKAY" {
                    self.parse_tree.push("#VARIABLE USE END".to_string());
                    self.next_token();
                    return;
                } else {
                    eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                    std::process::exit(1);
                }
        } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    fn parse_bold(&mut self){
        self.parse_tree_push();
        self.next_token();
        if !self.current_token.starts_with("#") {
                self.parse_text();
                if self.current_token == "#MKAY" {
                    self.parse_tree.push("#BOLD END".to_string());
                    self.next_token();
                    return;
                } else {
                    eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                    std::process::exit(1);
                }
        } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    fn parse_italics(&mut self){
        self.parse_tree_push();
        self.next_token();
        if !self.current_token.starts_with("#") {
                self.parse_text();
                if self.current_token == "#MKAY" {
                    self.parse_tree.push("#ITALICS END".to_string());
                    self.next_token();
                    return;
                } else {
                    eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                    std::process::exit(1);
                }
        } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    fn parse_list(&mut self){
        self.parse_tree_push();
        self.next_token();
        self.parse_list_items();
        return;
    }
    //similar setup to parse_comments, basically just keeps calling itself until #OIC is found, it then returns
    fn parse_list_items(&mut self){
        if self.current_token == "#OIC" {
            self.parse_tree.push("#LIST END".to_string());
            self.next_token();
            return
        } else if self.current_token == "#GIMMEH ITEM" {
            self.parse_tree_push();
            self.next_token();
            self.parse_inner_list();
            if self.current_token == "#MKAY" {
                    self.parse_tree.push("#LIST ITEM END".to_string());
                    self.next_token();
                    self.parse_list_items();
            } else {
                    eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                    std::process::exit(1);
            }
        } else {
                    eprintln!("syntax error: expected #GIMMEH ITEM but found {} instead", self.current_token);
                    std::process::exit(1);
        }
    }
    fn parse_inner_list(&mut self){
        if self.current_token == "#GIMMEH BOLD" {
            self.parse_bold();
            return;
        } else if self.current_token == "#GIMMEH ITALICS" {
            self.parse_italics();
            return;
        } else if !self.current_token.starts_with("#") {
            self.parse_text();
            return;
        } else if self.current_token == "#LEMME SEE" {
            self.parse_variable_use();
            return;
        } else {
            return;
        }
    }
    fn parse_audio(&mut self){
        self.parse_tree_push();
        self.next_token();
        if !self.current_token.starts_with("#") {
            self.parse_text();
            if self.current_token == "#MKAY" {
                self.parse_tree.push("#AUDIO END".to_string());
                self.next_token();
                return;
            } else {
                eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                std::process::exit(1);
            }
        } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    fn parse_video(&mut self){
        self.parse_tree_push();
        self.next_token();
        if !self.current_token.starts_with("#") {
            self.parse_text();
            if self.current_token == "#MKAY" {
                self.parse_tree.push("#VIDEO END".to_string());
                self.next_token();
                return;
            } else {
                eprintln!("syntax error: expected #MKAY but found {} instead", self.current_token);
                std::process::exit(1);
            }
        } else {
                eprintln!("syntax error: expected text but found {} instead", self.current_token);
                std::process::exit(1);
        }
    }
    fn parse_newline(&mut self){
        //this one pretty complicated all things considered
        self.parse_tree_push();
        self.next_token();
        return;
    }
    fn parse_text(&mut self){
        //this one was also pretty complicated
        self.parse_tree_push();
        self.next_token();
        return;
    }
}

//--------------------semantic analysis--------------------

pub struct LolcodeSemanticAnalyzer{
    pub output: String,
    pub parse_tree : Vec<String>,
    pub current_token : String,
}

impl LolcodeSemanticAnalyzer{
    pub fn new() -> Self{
        Self {
            output: String::new(),
            parse_tree: Vec::new(),
            current_token : String::new(),
        }
    }
}

pub trait SemanticAnalyzer{
    fn semantic_analysis(&mut self);
    fn next_token(&mut self);
    fn push_output(&mut self);
}

impl SemanticAnalyzer for LolcodeSemanticAnalyzer{
    fn next_token(&mut self){
        self.current_token = self.parse_tree.pop().unwrap();
    }
    fn push_output(&mut self){
        self.output.push_str(&self.current_token);
    }
    //basically just a very lengthy while loop, if a specific token is encountered, it outputs the corresponding html
    //the main meat of the semantic analysis is the static scoped variables, which i handled using 2 hashmaps, one for the body scope and one for the paragraph scope
    //when a variable is declared, the corresponding information is added to either the body or paragraph hashmap depending on which one the program is currently in (tracked w/ paragraph_scope bool)
    //when a paragraph ends, the paragraph hashmap is completely cleared out since that information will never be accessed again and will probably be replaced anyways
    //when a variable is called, if its in the body, it checks the body hashmap for the variable, if its there, cool, outputs its value, if not, gives an error and exits
    //if a variable is called in a paragraph, it checks the paragraph hashmap first, if its there, cool, outputs the value, if not, it checks the body one, if its there, cool, outputs the value, if not, gives an error and exits
    fn semantic_analysis(&mut self){
        self.parse_tree.reverse();
        let mut body_var: HashMap<String, String> = HashMap::new();
        let mut paragraph_var: HashMap<String, String> = HashMap::new();
        let mut paragraph_scope = false;
        let mut var_name: String;
        let mut var_value : String;
        self.next_token();
        loop{
            if self.current_token == "#HAI" {
                self.output.push_str("<html>");
                self.next_token();
            } else if self.current_token == "#KTHXBYE" {
                self.output.push_str("</html>");
                break;
            } else if self.current_token == "#GIMMEH BOLD" {
                self.output.push_str("<b>");
                self.next_token();
            } else if self.current_token == "#BOLD END" {
                self.output.push_str("</b>");
                self.next_token();
            } else if !self.current_token.starts_with("#") {
                self.push_output();
                self.next_token();
            } else if self.current_token == "#GIMMEH ITALICS" {
                self.output.push_str("<i>");
                self.next_token();
            } else if self.current_token == "#ITALICS END" {
                self.output.push_str("</i>");
                self.next_token();
            } else if self.current_token == "#GIMMEH SOUNDZ" {
                self.output.push_str("<audio controls> <source src=\"");
                self.next_token();
            } else if self.current_token == "#AUDIO END" {
                self.output.push_str("\"></audio>");
                self.next_token();
            } else if self.current_token == "#GIMMEH VIDZ" {
                self.output.push_str("<iframe src=\"");
                self.next_token();
            } else if self.current_token == "#VIDEO END" {
                self.output.push_str("\"/>");
                self.next_token();
            } else if self.current_token == "#GIMMEH NEWLINE" {
                self.output.push_str("<br>");
                self.next_token();
            } else if self.current_token == "#MAEK PARAGRAF" {
                self.output.push_str("<p>");
                self.next_token();
                paragraph_scope = true;
            } else if self.current_token == "#PARAGRAPH END" {
                self.output.push_str("</p>");
                self.next_token();
                paragraph_scope = false;
                paragraph_var.clear();
            } else if self.current_token == "#MAEK LIST" {
                self.output.push_str("<ul>");
                self.next_token();
            } else if self.current_token == "#GIMMEH ITEM" {
                self.output.push_str("<li>");
                self.next_token();
            } else if self.current_token == "#LIST ITEM END" {
                self.output.push_str("</li>");
                self.next_token();
            } else if self.current_token == "#LIST END" {
                self.output.push_str("</ul>");
                self.next_token();
            } else if self.current_token == "#OBTW" {
                self.output.push_str("<!--");
                self.next_token();
            } else if self.current_token == "#TLDR" {
                self.output.push_str("-->");
                self.next_token();
            } else if self.current_token == "#MAEK HEAD" {
                self.output.push_str("<head>");
                self.next_token();
            } else if self.current_token == "#GIMMEH TITLE" {
                self.output.push_str("<title>");
                self.next_token();
            } else if self.current_token == "#TITLE END" {
                self.output.push_str("</title>");
                self.next_token();
            } else if self.current_token == "#HEAD END" {
                self.output.push_str("</head>");
                self.next_token();
            } else if self.current_token == "#I HAZ" {
                self.next_token();
                var_name = self.current_token.clone();
                self.next_token();
                self.next_token();
                var_value = self.current_token.clone();
                self.next_token();
                self.next_token();
                if paragraph_scope {
                    paragraph_var.insert(var_name.to_string(), var_value.to_string());
                } else {
                    body_var.insert(var_name.to_string(), var_value.to_string());
                }
            } else if self.current_token == "#LEMME SEE" {
                self.next_token();
                let var_name = self.current_token.clone();
                if paragraph_scope {
                    if paragraph_var.contains_key(&var_name){
                        let final_var_value = paragraph_var.get(&var_name);
                        match final_var_value{
                            Some(value) => self.output.push_str(value),
                            //i wrote this line like this since it literally should never output, and thankfully it never has
                            None =>println!("whoops, this shouldnt output"),
                        }
                    }  else if body_var.contains_key(&var_name){
                        let final_var_value = body_var.get(&var_name);
                        match final_var_value{
                            Some(value) => self.output.push_str(value),
                            None =>println!("whoops, this shouldnt output"),
                        }
                    } else {
                        eprintln!("static semantic error: variable {} not found in scope", self.current_token);
                        std::process::exit(1);
                    }
                    }else {
                    if body_var.contains_key(&var_name){
                        let final_var_value = body_var.get(&var_name);
                        match final_var_value{
                            Some(value) => self.output.push_str(value),
                            None =>println!("whoops, this shouldnt output"),
                        }
                    } else {
                        eprintln!("static semantic error: variable {} not found in scope", self.current_token);
                        std::process::exit(1);
                    }
                }
                self.next_token();
                self.next_token();
                }
            }
        }
    }
//--------------------main--------------------

//the actual main driver behind everything
//first pulls text from .lol file, then runs compiler w/ pulled text
//eventually grabs outputted html from semantic analysis
//then writes content to corresponding html file and automatically opens chrome w/ the newly created html file
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let lolspeak_string = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });
    if !filename.to_lowercase().ends_with(".lol") {
        eprintln!("user error: the input file must have a .lol extension");
        std::process::exit(1);
    }
    if lolspeak_string.is_empty(){
        eprintln!("user error: the input file is empty");
        std::process::exit(1);
    }
    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&lolspeak_string);
    let raw_filename = filename.split('.').next().unwrap_or(filename);
    let html = compiler.syntaxer.output.clone();
    let html_filename = format!("{}.html", raw_filename);
    let mut file = match File::create(&html_filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file '{}': {}", html_filename, e);
            std::process::exit(1);
        }
    };
    if let Err(e) = file.write_all(html.as_bytes()) {
    eprintln!("Error writing to file: {}", e);
    }
    let current_dir = match env::current_dir() {
    Ok(dir) => dir,
    Err(e) => {
        eprintln!("Error getting current directory: {}", e);
        std::process::exit(1);
    }
    };
    let full_path = current_dir.join(html_filename);
    let file_path: String = full_path.to_string_lossy().into_owned();
    let chrome_path = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
    match Command::new(chrome_path)
        .arg(file_path)
        .spawn() 
    {
        Ok(_) => println!("Opening file in Chrome"),
        Err(e) => {
            eprintln!("Failed to open Chrome: {}", e);
            std::process::exit(1);
        }
    }
}