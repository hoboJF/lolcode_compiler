use std::env;
use std::fs;

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

pub struct LolcodeCompiler{
    lexer: LolcodeLexicalAnalyzer,
    current_token: String,
    syntaxer: LolcodeSyntaxAnalyzer,
}

impl LolcodeCompiler{

    pub fn new() -> Self{
        Self {
            lexer: LolcodeLexicalAnalyzer::new(""),
            current_token: String::new(),
            syntaxer: LolcodeSyntaxAnalyzer::new()
        }
    }

    }

impl Compiler for LolcodeCompiler{
    fn compile(&mut self, source: &str){
        self.lexer = LolcodeLexicalAnalyzer::new(source);
        self.lexer.tokenize();
        self.parse();
    }

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

    fn parse(&mut self){
        while !self.lexer.tokens.is_empty() {
        let mut tok = self.next_token();
        if tok.is_empty() {
            break;
        }
        if tok.trim().is_empty() {
            continue;
        }
        println!("Token: '{}'", tok);
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
}

impl LolcodeSyntaxAnalyzer{
    pub fn new() -> Self{
        Self {
            token_vector: Vec::new(),
            parse_tree: Vec::new(),
            current_token : String::new(),
        }
    }
}

impl SyntaxAnalyzer for LolcodeSyntaxAnalyzer{
    
    fn grab_token(&mut self, token: String){
        self.token_vector.push(token);
    }

    fn parse_tree_push(&mut self){
        self.parse_tree.push(self.current_token.clone());
    }

    fn next_token(&mut self){
        self.current_token = self.token_vector.pop().unwrap();
    }

    fn parse_lolcode(&mut self){
        self.token_vector.reverse();
        self.next_token();
        if (self.current_token != "#HAI"){
            eprintln!("syntax error: expected #HAI but found {} instead", self.current_token);
            std::process::exit(1);
        }
        self.parse_tree_push();
        self.next_token();
        self.parse_comment();
        self.parse_head();
        self.parse_body();
        if (self.current_token != "#KTHXBYE"){
            eprintln!("syntax error: expected #KTHXBYE but found {} instead", self.current_token);
            std::process::exit(1);
        }
        self.parse_tree_push();
        println!("{:?}", self.parse_tree);
        if(!self.token_vector.is_empty()){
            eprintln!("syntax error: extra tokens found after #KTHXBYE");
            std::process::exit(1);
        }
    }
    
    fn parse_head(&mut self){
        if (self.current_token == "#MAEK HEAD"){
            self.parse_tree_push();
            self.next_token();
            self.parse_title();
            if(self.current_token == "#OIC"){
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
        if (self.current_token == "#GIMMEH TITLE"){
            self.parse_tree_push();
            self.next_token();
            if (!self.current_token.starts_with("#")){
                self.parse_text();
                if(self.current_token == "#MKAY"){
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
    fn parse_comment(&mut self){
        if (self.current_token == "#TLDR"){
            self.parse_tree_push();
            self.next_token();
            if(!self.current_token.starts_with("#")){
                self.parse_text();
                if(self.current_token == "#OBTW"){
                    self.parse_tree_push();
                    self.next_token();
                    self.parse_comment();
                } else {
                    eprintln!("syntax error: expected #OBTW but found {} instead", self.current_token);
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
    fn parse_body(&mut self){
        if(self.current_token == "#KTHXBYE"){
            return
        } else if (!self.current_token.starts_with("#")){
            self.parse_text();
            self.parse_body();
        } else if (self.current_token == "#GIMMEH BOLD"){
            self.parse_bold();
            self.parse_body();
        } else if (self.current_token == "#GIMMEH ITALICS"){
            self.parse_italics();
            self.parse_body();
        } else if (self.current_token == "#GIMMEH NEWLINE"){
            self.parse_newline();
            self.parse_body();
        } else if (self.current_token == "#MAEK LIST"){
            self.parse_list();
            self.parse_body();
        } else if (self.current_token == "#GIMMEH NEWLINE"){
            self.parse_newline();
            self.parse_body();
        }
    }
    fn parse_paragraph(&mut self){

    }
    fn parse_inner_paragraph(&mut self){

    }
    fn parse_inner_text(&mut self){

    }
    fn parse_variable_define(&mut self){

    }
    fn parse_variable_use(&mut self){
        return;
    }
    fn parse_bold(&mut self){
        self.parse_tree_push();
        self.next_token();
        if(!self.current_token.starts_with("#")){
                self.parse_text();
                if (self.current_token == "#MKAY"){
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
        if(!self.current_token.starts_with("#")){
                self.parse_text();
                if (self.current_token == "#MKAY"){
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
    fn parse_list_items(&mut self){
        if (self.current_token == "#OIC"){
            self.parse_tree.push("#LIST END".to_string());
            self.next_token();
            return
        } else if (self.current_token == "#GIMMEH ITEM"){
            self.parse_tree_push();
            self.next_token();
            self.parse_inner_list();
            if (self.current_token == "#MKAY"){
                    self.parse_tree.push("#LIST ITEM END".to_string());
                    self.next_token();
                    self.parse_list_items();
            }
        } else {
                    eprintln!("syntax error: expected #GIMMEH ITEM but found {} instead", self.current_token);
                    std::process::exit(1);
        }
    }
    fn parse_inner_list(&mut self){
        if(self.current_token == "#GIMMEH BOLD"){
            self.parse_bold();
            return;
        } else if (self.current_token == "#GIMMEH ITALICS"){
            self.parse_italics();
            return;
        } else if (!self.current_token.starts_with("#")){
            self.parse_text();
            return;
        } else if (self.current_token == "#LEMME SEE"){
            self.parse_variable_use();
            return;
        } else {
            return;
        }
    }
    fn parse_audio(&mut self){

    }
    fn parse_video(&mut self){

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

//--------------------main--------------------

fn main() {
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
    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&lolspeak_string);
}
