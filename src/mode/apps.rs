use super::Mode;
use crate::{item::Item, util};
use log::{debug, info};
use rayon::prelude::*;
use std::io::BufRead;
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path, time::Instant};

pub struct AppsMode {
    options: Vec<Item>,
}

impl AppsMode {
    pub fn load() -> Self {
        let start = Instant::now();
        let apps = load_apps();
        info!("Loaded {} apps in {:?}", apps.len(), start.elapsed());
        Self { options: apps }
    }
}

impl Mode for AppsMode {
    fn name(&self) -> &str {
        "Applications"
    }

    fn options(&mut self) -> Vec<Item> {
        self.options.clone()
    }
}

fn load_apps() -> Vec<Item> {
    let start = Instant::now();
    let items = util::find_files_from_env("XDG_DATA_DIRS", &|path| {
        Some(OsStr::new("desktop")) == path.extension()
    });
    info!("Loaded {} env in {:?}", items.len(), start.elapsed());

    let mut items = items
        .into_par_iter()
        .filter_map(|path| read_desktop_file(&path))
        .collect::<Vec<Item>>();
    items.sort_unstable_by_key(|a| a.text());
    items
}

// Per the Desktop Entry Specification: https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html
fn read_desktop_file(path: &Path) -> Option<Item> {
    let mut name = None;
    let mut exec = None;

    let file = File::open(path).ok()?;
    for line in BufReader::new(file).lines() {
        let line = line.ok()?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=').map(|(k, v)| (k.trim(), v.trim())) {
            match key {
                "Name" => name = Some(value.to_string()),
                "Exec" => exec = Some(value.to_string()),
                _ => {}
            }
        }
        if name.is_some() && exec.is_some() {
            break;
        }
    }
    let name = unescape_string(&name?);
    let exec = exec?;
    let exec_args = exec_args(&exec);
    let exec = exec_args
        .into_iter()
        .filter_map(|a| {
            if let DesktopArg::Arg(s) = a {
                Some(s)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    debug!("Desktop file: {} -> {}", name, exec);
    Some(Item::Exec { name, exec })
}

#[derive(Debug, Clone, PartialEq)]
enum DesktopArg {
    Arg(String),
    File,
    Files,
    Url,
    Urls,
}

/// Parse the exec string according to the Desktop Entry Specification
// TODO: Handle escaping of %%
fn exec_args(exec: &str) -> Vec<DesktopArg> {
    let mut args = Vec::new();
    for arg in unquote_args(&unescape_string(exec)) {
        if arg.starts_with('%') && !arg.starts_with("%%") {
            let code = &arg[1..];
            if Some(a) = match code.chars().next() {
                Some('f') => Some(DesktopArg::File),
                Some('F') => Some(DesktopArg::Files),
                Some('u') => Some(DesktopArg::Url),
                Some('U') => Some(DesktopArg::Urls),
                _ => None,
            } {
                args.push(a);
                continue;
            }
        }
        args.push(DesktopArg::Arg(arg));
    }
    args
}

/// Unescape \s, \n, \t, \r, and \\
fn unescape_string(str: &str) -> String {
    let mut iter = str.chars().peekable();
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
    return result;
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
    return args;
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
        assert_eq!(exec_args("foo"), vec![DesktopArg::Arg("foo".to_string())]);
        assert_eq!(
            exec_args("fooview %F"),
            vec![DesktopArg::Arg("fooview".to_string()), DesktopArg::Files]
        );
        assert_eq!(
            exec_args("foo %f bar %F baz %u qux %U"),
            vec![
                DesktopArg::Arg("foo".to_string()),
                DesktopArg::File,
                DesktopArg::Arg("bar".to_string()),
                DesktopArg::Files,
                DesktopArg::Arg("baz".to_string()),
                DesktopArg::Url,
                DesktopArg::Arg("qux".to_string()),
                DesktopArg::Urls
            ]
        );
    }
}
