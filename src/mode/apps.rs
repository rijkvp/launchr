use super::Mode;
use crate::item::{Action, Exec};
use crate::winit_app::EventHandle;
use crate::{file_finder, item::Item};
use rayon::prelude::*;
use std::collections::HashSet;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path, time::Instant};

pub struct AppsMode {
    options: Arc<Mutex<Vec<Item>>>,
}

impl AppsMode {
    pub fn load() -> Self {
        Self {
            options: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Mode for AppsMode {
    fn name(&self) -> &str {
        "Apps"
    }

    fn run(&mut self, event_handle: EventHandle) {
        let options = self.options.clone();
        thread::spawn(move || {
            let items = load_desktop_files();
            event_handle.send_update();
            let mut options_mut = options.lock().unwrap();
            *options_mut = items;
        });
    }

    fn update(&mut self, input: &str) -> Vec<Item> {
        let items = self.options.lock().unwrap().clone();
        super::fuzzy_match(input, &items)
    }
}

pub fn load_desktop_files() -> Vec<Item> {
    let mut timer = Instant::now();
    let mut dirs = file_finder::get_dirs_from_env("XDG_DATA_DIRS");
    if let Some(desktop_dir) = dirs::desktop_dir() {
        dirs.push(desktop_dir);
    }
    if let Some(data_dir) = dirs::data_dir() {
        dirs.push(data_dir.join("applications"));
    }
    let desktop_files = file_finder::find_files_from_dirs(&dirs, &|path| {
        Some(OsStr::new("desktop")) == path.extension()
    });
    log::info!(
        "found {} desktop files in {:?}",
        desktop_files.len(),
        timer.elapsed()
    );

    timer = Instant::now();
    let entries = desktop_files
        .into_par_iter()
        .filter_map(|path| read_desktop_file(&path))
        .collect::<Vec<DesktopEntry>>();
    log::info!(
        "parsed {} desktop files in {:?}",
        entries.len(),
        timer.elapsed()
    );
    let set: HashSet<DesktopEntry> = HashSet::from_iter(entries);
    log::info!("deduplicated desktop files: {}", set.len());
    let mut items: Vec<Item> = set.into_iter().map(Item::from).collect();
    items.sort_by(|a, b| a.text.cmp(&b.text));
    items
}

// Per the Desktop Entry Specification: https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
fn read_desktop_file(path: &Path) -> Option<DesktopEntry> {
    let mut name_str = None;
    let mut exec_str = None;

    let file = File::open(path).ok()?;
    for line in BufReader::new(file).lines() {
        let line = line.ok()?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=').map(|(k, v)| (k.trim(), v.trim())) {
            match key {
                "Name" => name_str = Some(value.to_string()),
                "Exec" => exec_str = Some(value.to_string()),
                _ => {}
            }
        }
        if name_str.is_some() && exec_str.is_some() {
            break;
        }
    }
    let name = unescape_string(&name_str?);
    let exec_args = ExecKey::parse(&exec_str?);
    let exec = exec_args.expand();

    log::debug!("read desktop file: {} -> {:?}", name, exec);
    Some(DesktopEntry::new(name, exec))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DesktopEntry {
    name: String,
    exec: Exec,
}

impl DesktopEntry {
    fn new(name: String, exec: Exec) -> Self {
        Self { name, exec }
    }
}

impl From<DesktopEntry> for Item {
    fn from(value: DesktopEntry) -> Self {
        Item::new(value.name, Action::Exec { exec: value.exec })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecKey(Vec<ExecArg>);

impl ExecKey {
    /// Parse the exec string according to the Desktop Entry Specification
    pub fn parse(s: &str) -> Self {
        Self(
            unquote_args(&unescape_string(s))
                .into_iter()
                .flat_map(|a| field_codes(&a))
                .collect(),
        )
    }

    /// Expand field codes without data
    fn expand(self) -> Exec {
        let parts = self
            .0
            .into_iter()
            .filter_map(|arg| match arg {
                ExecArg::Arg(s) => Some(s),
                _ => None,
            })
            .collect::<Vec<_>>();
        Exec {
            program: parts[0].clone(),
            args: parts[1..].to_vec(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ExecArg {
    Arg(String),
    File,
    Files,
    Url,
    Urls,
}

// Parse field code in Exec key according to the Desktop Entry Specification
fn field_codes(input: &str) -> Vec<ExecArg> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut iter = input.chars();
    while let Some(c) = iter.next() {
        if c == '%' {
            if let Some(next) = iter.next() {
                if next == '%' {
                    current.push('%');
                    continue;
                } else if !current.is_empty() {
                    args.push(ExecArg::Arg(current));
                    current = String::new();
                }
                if let Some(arg) = match next {
                    'f' => Some(ExecArg::File),
                    'F' => Some(ExecArg::Files),
                    'u' => Some(ExecArg::Url),
                    'U' => Some(ExecArg::Urls),
                    _ => None,
                } {
                    args.push(arg);
                }
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        args.push(ExecArg::Arg(current));
    }
    args
}

/// Unescape \s, \n, \t, \r, and \\
fn unescape_string(input: &str) -> String {
    let mut iter = input.chars().peekable();
    let mut result = String::new();
    while let Some(c) = iter.next() {
        if c == '\\' {
            if let Some(replacement) = match iter.peek() {
                Some('s') => Some(' '),
                Some('n') => Some('\n'),
                Some('t') => Some('\t'),
                Some('r') => Some('\r'),
                Some('\\') => Some('\\'),
                _ => None,
            } {
                result.push(replacement);
                iter.next();
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Unquote a string argument according to the Desktop Entry Specification
fn unquote_args(input: &str) -> Vec<String> {
    let mut iter = input.chars().peekable();
    let mut in_quote = false;
    let mut current = String::new();
    let mut args = Vec::new();
    while let Some(c) = iter.next() {
        match c {
            ' ' if !in_quote => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            '\\' if in_quote => {
                if let Some(next) = iter.peek() {
                    if let '"' | '`' | '$' | '\\' = next {
                        current.push(*next);
                        iter.next();
                    }
                }
            }
            '"' if !in_quote => in_quote = true,
            '"' if in_quote => {
                args.push(current);
                current = String::new();
                in_quote = false;
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape() {
        assert_eq!(unescape_string("foo"), "foo");
        assert_eq!(unescape_string("foo\\sbar"), "foo bar");
        assert_eq!(unescape_string("foo\\nbar"), "foo\nbar");
        assert_eq!(unescape_string("foo\\tbar"), "foo\tbar");
        assert_eq!(unescape_string("foo\\r\\n"), "foo\r\n");
        assert_eq!(unescape_string("foo\\\\bar"), "foo\\bar");
        assert_eq!(unescape_string("foo\\bar"), "foobar");
    }

    #[test]
    fn test_unquote_args() {
        assert_eq!(unquote_args("foo"), vec!["foo"]);
        assert_eq!(unquote_args("foo bar"), vec!["foo", "bar"]);
        assert_eq!(unquote_args("foo            bar"), vec!["foo", "bar"]);
        assert_eq!(unquote_args("foo \"bar baz\""), vec!["foo", "bar baz"]);
        assert_eq!(unquote_args("foo \"bar\" baz"), vec!["foo", "bar", "baz"]);
        assert_eq!(unquote_args("foo bar \"baz\""), vec!["foo", "bar", "baz"]);
        assert_eq!(unquote_args("foo \"$BAR\""), vec!["foo", "$BAR"]);
        assert_eq!(unquote_args("\"\\foo\" bar baz"), vec!["foo", "bar", "baz"]);
        assert_eq!(
            unquote_args("foo \"bar\\ \" baz"),
            vec!["foo", "bar ", "baz"]
        );
        assert_eq!(unquote_args("\"quoted\""), vec!["quoted"]);
        assert_eq!(
            unquote_args(r#""escaped chars \" \` \$ \\""#),
            vec!["escaped chars \" ` $ \\"]
        );
    }

    #[test]
    fn test_exec_args() {
        assert_eq!(
            ExecKey::parse("foo"),
            ExecKey(vec![ExecArg::Arg("foo".to_string())])
        );
        assert_eq!(
            ExecKey::parse("foo %% bar"),
            ExecKey(vec![
                ExecArg::Arg("foo".to_string()),
                ExecArg::Arg("%".to_string()),
                ExecArg::Arg("bar".to_string())
            ])
        );
        assert_eq!(
            ExecKey::parse("foo %% bar"),
            ExecKey(vec![
                ExecArg::Arg("foo".to_string()),
                ExecArg::Arg("%".to_string()),
                ExecArg::Arg("bar".to_string())
            ])
        );
        assert_eq!(
            ExecKey::parse("fooview %F"),
            ExecKey(vec![ExecArg::Arg("fooview".to_string()), ExecArg::Files])
        );
        assert_eq!(
            ExecKey::parse("invalid %! invalid %"), // Invalid field codes are ignored
            ExecKey(vec![
                ExecArg::Arg("invalid".to_string()),
                ExecArg::Arg("invalid".to_string())
            ])
        );
        assert_eq!(
            ExecKey::parse("foo %f bar %F baz %u qux %U"),
            ExecKey(vec![
                ExecArg::Arg("foo".to_string()),
                ExecArg::File,
                ExecArg::Arg("bar".to_string()),
                ExecArg::Files,
                ExecArg::Arg("baz".to_string()),
                ExecArg::Url,
                ExecArg::Arg("qux".to_string()),
                ExecArg::Urls
            ])
        );
    }
}
