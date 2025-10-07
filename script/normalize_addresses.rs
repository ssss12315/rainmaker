use std::{collections::HashSet, env, fs, io::{self, Write}, path::PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: normalize_addresses <input> <output>");
        std::process::exit(1);
    }
    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);

    let content = fs::read_to_string(&input)?;
    let mut seen = HashSet::new();
    let mut out = String::new();

    for line in content.lines() {
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') { continue; }
        // keep CSV line intact; de-duplicate by the left-most address
        let key = s.split(',').next().unwrap_or(s).trim().to_lowercase();
        if seen.insert(key) {
            out.push_str(s);
            out.push('\n');
        }
    }

    let mut f = fs::File::create(&output)?;
    f.write_all(out.as_bytes())?;
    eprintln!(" Wrote normalized addresses to {}", output.display());
    Ok(())
}
