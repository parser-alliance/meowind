use std::path::PathBuf;

pub struct MeowindScriptSource<'a> {
    pub path: PathBuf,
    pub contents: &'a String,
    pub chars: Vec<char>,
    pub lines: Vec<&'a str>,
}

impl<'a> MeowindScriptSource<'a> {
    pub fn new(path: PathBuf, contents: &'a String) -> MeowindScriptSource<'a> {
        MeowindScriptSource {
            path,
            contents,
            chars: contents.chars().collect(),
            lines: contents.lines().collect(),
        }
    }
}

pub struct MeowindArguments {
    pub path: PathBuf,
}
