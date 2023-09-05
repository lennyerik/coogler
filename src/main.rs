use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Location {
    file_name: String,
    line: u32,
    column: u32,
}

#[derive(Debug)]
struct Function {
    name: String,
    location: Location,
    return_type: String,
    parameters: Vec<(String, Option<String>)>,
}

fn normalise_type(type_str: &str) -> String {
    // TODO: This might not be the best way to add spaces between consecutive "*" characters...
    type_str.replace("**", "* *").replace("**", "* *")
}

impl Function {
    fn normalised_signature(&self) -> String {
        let param_reprs: Vec<String> = self
            .parameters
            .iter()
            .map(|(typ, _)| normalise_type(typ))
            .collect();

        format!(
            "{} ( {} )",
            normalise_type(&self.return_type),
            param_reprs.join(" , ")
        )
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let param_reprs: Vec<String> = self
            .parameters
            .iter()
            .map(|(typ, name)| {
                name.as_ref()
                    .map_or_else(|| typ.clone(), |name| typ.clone() + " " + name)
            })
            .collect();

        f.write_fmt(format_args!(
            "{}:{}:{}: {} :: {}({})",
            self.location.file_name,
            self.location.line,
            self.location.column,
            self.name,
            self.return_type,
            param_reprs.join(", ")
        ))
    }
}

fn normalise_query(index: &clang::Index, query_string: &str) -> Result<String, String> {
    let unsaved_path = Path::new("<query>");
    let unsaved = clang::Unsaved::new(unsaved_path, query_string);

    let parsed = index
        .parser(unsaved_path)
        .skip_function_bodies(true)
        .single_file_parse(true)
        .incomplete(true)
        .arguments(&["-xc", "-E", "-w", "-ferror-limit=1"])
        .unsaved(&[unsaved])
        .parse()?;

    let file = parsed
        .get_file(unsaved_path)
        .expect("libclang did not return the temporary (unsaved) user query file");
    let file_range = {
        let start = file.get_offset_location(0);
        let end = file.get_offset_location(
            // It is unlikely that someone is ever going to enter a string this large
            u32::try_from(query_string.len()).expect("Query string is too long"),
        );
        clang::source::SourceRange::new(start, end)
    };

    let tokens = file_range.tokenize();
    Ok(tokens
        .iter()
        .map(clang::token::Token::get_spelling)
        .collect::<Vec<_>>()
        .join(" "))
}

fn main() -> Result<(), String> {
    let mut args = std::env::args();

    let source_path = args.nth(1).unwrap_or_else(|| usage());
    let search_string = args.next().unwrap_or_else(|| usage());

    let clang = clang::Clang::new().expect("Failed to get libclang instance");
    let index = clang::Index::new(&clang, false, false);

    let mut clang_args = clang_c_include_path_args().unwrap_or_else(|err| {
        eprintln!("Failed to get default C include paths: {err}");
        Vec::new()
    });
    clang_args.push("-xc".into());
    let translation_unit = index
        .parser(PathBuf::from(&source_path))
        .arguments(&clang_args)
        .skip_function_bodies(true)
        .parse()?;

    let error_occurred = translation_unit
        .get_diagnostics()
        .iter()
        .filter(|d| d.get_severity() >= clang::diagnostic::Severity::Error)
        .inspect(|d| eprintln!("{}", d.get_text()))
        .count()
        > 0;
    if error_occurred {
        return Err("Clang failed to parse the file provided".into());
    }

    let normalised_query = normalise_query(&index, &search_string)?;

    let mut functions = Vec::new();
    translation_unit.get_entity().visit_children(|cur, _| {
        if cur.get_kind() != clang::EntityKind::FunctionDecl {
            return clang::EntityVisitResult::Continue;
        }

        let children = cur.get_children();
        let parameters = children
            .iter()
            .filter(|child| child.get_kind() == clang::EntityKind::ParmDecl)
            .map(|param| {
                let param_name = param.get_name();
                let param_type = param
                    .get_type()
                    .expect("Found parameter without a type")
                    .get_display_name();
                (param_type, param_name)
            })
            .collect();

        let (file_name, line, column) = cur
            .get_location()
            .expect("Found function at unknown location")
            .get_presumed_location();

        let location = Location {
            file_name,
            line,
            column,
        };

        functions.push(Function {
            name: cur.get_name().expect("Found function without a name"),
            location,
            return_type: cur
                .get_result_type()
                .expect("Found function without a return type")
                .get_display_name(),
            parameters,
        });

        clang::EntityVisitResult::Continue
    });

    let mut distances: Vec<(usize, usize)> = Vec::with_capacity(functions.len());
    for (i, func) in functions.iter().enumerate() {
        distances.push((
            levenshtein_distance(&func.normalised_signature(), &normalised_query),
            i,
        ));
    }

    distances.sort_by_key(|d| d.0);
    for (_, index) in distances.into_iter().take(20) {
        println!("{}", functions[index]);
    }

    Ok(())
}

fn usage() -> ! {
    eprintln!(
        "Usage: {} <SOURCE_FILE> <FUNCTION_SIGNATURE>",
        std::env::args()
            .next()
            .unwrap_or_else(|| "./coogler".into())
    );
    std::process::exit(1)
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![0_usize; (n + 1) * (m + 1)];

    for i in 0..=n {
        dp[i * (m + 1)] = i;
    }

    // An iterator for loop makes the linter happy
    for (j, d) in dp.iter_mut().enumerate().take(m + 1) {
        *d = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            dp[i * (m + 1) + j] = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[(i - 1) * (m + 1) + (j - 1)]
            } else {
                dp[i * (m + 1) + (j - 1)]
                    .min(dp[(i - 1) * (m + 1) + j])
                    .min(dp[(i - 1) * (m + 1) + (j - 1)])
                    + 1
            }
        }
    }

    *dp.last().unwrap_or(&a.len().max(b.len()))
}

fn clang_c_include_path_args() -> Result<Vec<String>, String> {
    let cc_result = std::process::Command::new("cc")
        .args(["-xc", "-E", "-v", "-"])
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|_| "Failed to execute the default C compiler (cc)")?;

    let cc_output = std::str::from_utf8(&cc_result.stderr).unwrap_or("<Invalid UTF-8>");
    if !cc_result.status.success() {
        return Err(format!(
            "The default C compiler (cc) failed with message:\n{cc_output}"
        ));
    }

    Ok(cc_output
        .lines()
        .map(str::trim)
        .skip_while(|line| *line != r#"#include "..." search starts here:"#)
        .skip(1)
        .filter(|line| *line != "#include <...> search starts here:")
        .take_while(|line| *line != "End of search list.")
        .flat_map(|path| ["-isystem", path])
        .map(String::from)
        .collect())
}
