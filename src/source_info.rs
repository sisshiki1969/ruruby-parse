use std::path::PathBuf;

pub type SourceInfoRef = std::rc::Rc<SourceInfo>;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Loc(pub usize, pub usize);

impl Loc {
    pub fn merge(&self, loc: Loc) -> Self {
        use std::cmp::*;
        Loc(min(self.0, loc.0), max(self.1, loc.1))
    }
}

/// This struct holds infomation of a certain line in the code.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    /// line number. (the first line is 1)
    pub line_no: usize,
    /// byte position of the line top in the code.
    pub top: usize,
    /// byte position of the line end in the code.
    pub end: usize,
}

impl Line {
    fn new(line_no: usize, top: usize, end: usize) -> Self {
        Line { line_no, top, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceInfo {
    /// directory path of the source code.
    pub path: PathBuf,
    /// source code text.
    pub code: String,
}

impl Default for SourceInfo {
    fn default() -> Self {
        SourceInfo::new(PathBuf::default(), "")
    }
}

//
// public interface
//
impl SourceInfo {
    /// Get file name.
    pub fn get_file_name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    /// Show the location of *loc* in source text.
    pub fn show_loc(&self, loc: &Loc) {
        eprint!("{}", self.get_location(loc));
    }

    /// Return a string represents the location of `loc` in the source code using '^^^'.
    pub fn get_location(&self, loc: &Loc) -> String {
        if self.code.is_empty() {
            return "(internal)".to_string();
        }
        let code = self.code.clone() + " ";
        let mut res_string = String::new();
        let lines = self.get_lines(loc);
        let mut found = false;
        for line in &lines {
            if !found {
                res_string += &format!("{}:{}\n", self.path.to_string_lossy(), line.line_no);
                found = true;
            };

            let start = line.top;
            let mut end = line.end;
            if self.get_next_char(end) == Some('\n') && end > 0 {
                end -= 1
            }
            res_string += &code[start..=end];
            res_string.push('\n');
            use std::cmp::*;
            let lead = if loc.0 <= line.top {
                0
            } else {
                console::measure_text_width(&code[start..loc.0])
            };
            let range_start = max(loc.0, line.top);
            let range_end = min(loc.1, line.end);
            let length = console::measure_text_width(&code[range_start..=range_end]);
            res_string += &" ".repeat(lead);
            res_string += &"^".repeat(length);
            res_string += "\n";
        }

        if !found {
            res_string += "NOT FOUND\n";
            let line = match lines.last() {
                Some(line) => (line.line_no + 1, line.end + 1, loc.1),
                None => (1, 0, loc.1),
            };
            let lead = console::measure_text_width(&code[line.1..loc.0]);
            let length = console::measure_text_width(&code[loc.0..loc.1]);
            let is_cr = loc.1 >= code.len() || self.get_next_char(loc.1) == Some('\n');
            res_string += &format!("{}:{}\n", self.path.to_string_lossy(), line.0);
            res_string += if !is_cr {
                &code[line.1..=loc.1]
            } else {
                &code[line.1..loc.1]
            };
            res_string += &" ".repeat(lead);
            res_string += &"^".repeat(length + 1);
            res_string += "\n";
        }
        res_string
    }
}

impl SourceInfo {
    pub fn new(path: impl Into<PathBuf>, code: impl Into<String>) -> Self {
        SourceInfo {
            path: path.into(),
            code: code.into(),
        }
    }

    fn get_next_char(&self, pos: usize) -> Option<char> {
        self.code[pos..].chars().next()
    }

    fn get_lines(&self, loc: &Loc) -> Vec<Line> {
        let mut line_top = 0;
        let code = self.code.clone() + " ";
        let code_len = code.len();
        let mut lines: Vec<_> = code
            .char_indices()
            .filter(|(_, ch)| *ch == '\n')
            .map(|(pos, _)| pos)
            .enumerate()
            .map(|(idx, pos)| {
                let top = line_top;
                line_top = pos + 1;
                Line::new(idx + 1, top, pos)
            })
            .filter(|line| line.end >= loc.0 && line.top <= loc.1)
            .collect();
        if line_top < code_len && loc.0 <= code_len - 1 && line_top <= loc.1 {
            lines.push(Line::new(lines.len() + 1, line_top, code_len - 1));
        }
        lines
    }
}
