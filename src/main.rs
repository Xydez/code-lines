use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::Path;
use walkdir::WalkDir;

struct CodeDir {
    path: String,
    lines: usize,
}

fn count_lines_of_code(path: &Path) -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(path)?
        .chars()
        .filter(|c| c == &'\n')
        .count())
}

fn main() {
    let matches = clap::App::new("code-lines")
        .version(clap::crate_version!())
        .author("Xydez <thexydez@gmail.com>")
        .arg(
            clap::Arg::new("exclude-dir")
                .short('e')
                .long("exclude-dir")
                .value_name("DIR")
                .help("Excludes files within a directory")
                .number_of_values(1)
                .multiple_occurrences(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Only display the lines of code and nothing else"),
        )
        .arg(
            clap::Arg::new("DIR")
                .help("The directory to search")
                .index(1),
        )
        .get_matches();

    let mut languages = HashMap::<&str, Vec<&str>>::new();
    languages.insert("Python", vec!["py", "pyi", "pyc", "pyw"]);
    languages.insert("JavaScript", vec!["js", "jsx"]);
    languages.insert("Java", vec!["java"]);
    languages.insert("C/C++", vec!["cpp", "cxx", "hpp", "hxx", "c", "cc", "h"]);
    languages.insert("C#", vec!["cs", "csx"]);
    languages.insert(
        "PHP",
        vec![
            "php", "phtml", "php3", "php4", "php5", "php7", "phps", "php-s", "pht", "phar",
        ],
    );
    languages.insert("Perl", vec!["plx", "pl", "pm", "xs", "t", "pod"]);
    languages.insert("Rust", vec!["rs"]);
    languages.insert("HTML", vec!["html", "htm"]);
    languages.insert("Shell script", vec!["sh"]);
    languages.insert("TypeScript", vec!["ts", "tsx"]);
    languages.insert("Batch script", vec!["bat", "cmd"]);
    languages.insert("R", vec!["r"]);
    languages.insert("Objective-C", vec!["m", "mm", "M"]);
    languages.insert("Swift", vec!["swift"]);
    languages.insert("Kotlin", vec!["kt"]);
    languages.insert("Go", vec!["go"]);
    languages.insert("Ruby", vec!["rb"]);
    languages.insert("Scala", vec!["scala"]);
    languages.insert("Ada", vec!["ada"]);
    languages.insert("Dart", vec!["dart"]);
    languages.insert("Lua", vec!["lua"]);
    languages.insert("Perl", vec!["pl"]);
    languages.insert("Groovy", vec!["gradle", "groovy"]);
    languages.insert("Julia", vec!["jl"]);
    languages.insert("Cobol", vec!["cobol"]);
    languages.insert("Pascal", vec!["pas"]);
    languages.insert("Haskell", vec!["hs"]);
    languages.insert("GDScript", vec!["gd"]);

    let mut excluded_dirs = Vec::<&str>::new();
    excluded_dirs.push("target");
    excluded_dirs.push("build");
    excluded_dirs.push("bin");
    excluded_dirs.push("dist");
    excluded_dirs.push(".git");

    match matches.values_of("exclude-dir") {
        Some(values) => {
            for val in values {
                excluded_dirs.push(val);
            }
        }
        None => (),
    }

    let dir = match matches.value_of("DIR") {
        Some(val) => Path::new(val),
        None => Path::new("."),
    };

    if !dir.exists() {
        eprintln!("Directory \"{}\" not found.", dir.to_str().unwrap());
        return;
    }

    let quiet = matches.is_present("quiet");

    let size: (usize, usize) = match term_size::dimensions() {
        Some((w, h)) => (w, h),
        None => (80, 24),
    };

    if !quiet {
        println!("code-lines v0.1.0");
    }

    let mut line_count = HashMap::<&str, usize>::new();
    let mut dirs = Vec::<CodeDir>::new();

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        let mut excluded = false;
        for part in path.iter() {
            for dir in &excluded_dirs {
                if part.to_str().unwrap().eq(*dir) {
                    excluded = true;
                    break;
                }
            }

            if excluded {
                break;
            }
        }

        if excluded {
            continue;
        }

        if let Some(ext) = path.extension().map(|v| v.to_str().unwrap()) {
            if let Some((lang_name, _)) = languages
                .iter()
                .find(|(_, lang_exts)| lang_exts.contains(&ext))
            {
                match count_lines_of_code(&path) {
                    Ok(lines) => {
                        let dir = CodeDir {
                            path: String::from(path.to_str().unwrap()),
                            lines,
                        };

                        dirs.push(dir);

                        match line_count.get_mut(lang_name) {
                            Some(prev_lines) => *prev_lines += lines,
                            None => {
                                line_count.insert(lang_name, lines);
                            }
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        }
    }

    if !quiet {
        println!("\nFiles ({})", dirs.len());
        println!("{}", "=".repeat(size.0 - 1));

        dirs.sort_by(|a, b| b.lines.cmp(&a.lines));
        for dir in dirs {
            if dir.lines == 0 {
                continue;
            }

            let len = size.0 - dir.lines.to_string().len() - 2;

            let mut tmp = dir.path.chars().rev().collect::<String>();
            tmp.truncate(len);
            tmp = tmp.chars().rev().collect::<String>();

            println!("{:<2$} {}", tmp, dir.lines, len);
        }

        println!("\nLanguages");
        println!("{}", "=".repeat(size.0 - 1));

        let mut sorted_line_count = Vec::from_iter(line_count.iter());
        sorted_line_count.sort_by(|a, b| b.1.cmp(&a.1));

        let mut total = 0;
        for (key, value) in sorted_line_count {
            println!(
                "{:<2$} {}",
                key,
                value,
                size.0 - value.to_string().len() - 2
            );
            total += value;
        }

        println!("{}\n", "-".repeat(size.0 - 1));
        println!(
            "{:<2$} {}",
            "Total:",
            total,
            size.0 - total.to_string().len() - 2
        );
    } else {
        let mut total = 0;
        for (_, value) in line_count {
            total += value;
        }

        println!("{}", total);
    }
}
