use std::collections::HashMap;
use walkdir::WalkDir;
use std::path::Path;

fn count_lines_of_code(path: &Path) -> u32
{
	let code = std::fs::read_to_string(path).expect("Failed to read file!");

	let mut lines = 0;
	for c in code.chars()
	{
		if c == '\n'
		{
			lines += 1;
		}
	}

	return lines;
}

fn main()
{
	let mut languages = HashMap::<&str, Vec<&str>>::new();
	languages.insert("Python", ["py", "pyi", "pyc", "pyw"].to_vec());
	languages.insert("JavaScript", ["js"].to_vec());
	languages.insert("Java", ["java"].to_vec());
	languages.insert("C/C++", ["cpp", "cxx", "hpp", "hxx", "c", "cc", "h"].to_vec());
	languages.insert("C#", ["cs", "csx"].to_vec());
	languages.insert("PHP", ["php", "phtml", "php3", "php4", "php5", "php7", "phps", "php-s", "pht", "phar"].to_vec());
	languages.insert("Perl", ["plx", "pl", "pm", "xs", "t", "pod"].to_vec());
	languages.insert("Rust", ["rs"].to_vec());

	let mut excluded_dirs = Vec::<&str>::new();
	excluded_dirs.push("target");
	excluded_dirs.push("build");
	excluded_dirs.push(".git");

	println!("code-lines v0.1.0");

	let mut line_count = HashMap::<&str, u32>::new();
	
	for entry in WalkDir::new(".")
	{
		let entry = entry.unwrap();
		let path = entry.path();

		let mut excluded = false;
		for part in path.iter()
		{
			for dir in &excluded_dirs
			{
				if part.to_str().unwrap().eq(*dir)
				{
					excluded = true;
					break;
				}
			}

			if excluded
			{
				break;
			}
		}

		if excluded
		{
			continue;
		}

		let ext = path.extension();
		match ext
		{
			Some(ext) => {
				for (lang, exts) in &languages
				{
					for lang_ext in exts
					{
						if ext.to_str().unwrap().eq(*lang_ext)
						{
							let lines = count_lines_of_code(path);
							println!("{} ({}, {} lines)", path.display(), lang, lines);

							let prev = line_count.get(lang);
							match prev
							{
								Some(val) => { line_count.insert(lang, lines + val); },
								None => { line_count.insert(lang, lines); }
							}
						}
					}
				}
			},
			None => ()
		}
	}

	for (key, value) in line_count
	{
		println!("{:<12} {}", key, value);
	}
}
