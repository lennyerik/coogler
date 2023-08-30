use std::path::PathBuf;

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
            "{}:{}:{}: {} {}({})",
            self.location.file_name,
            self.location.line,
            self.location.column,
            self.return_type,
            self.name,
            param_reprs.join(", ")
        ))
    }
}

fn main() -> Result<(), String> {
    let mut args = std::env::args();

    let source_path = args.nth(1).unwrap_or_else(|| usage());
    let _search_string = args.next().unwrap_or_else(|| usage());

    let clang = clang::Clang::new().expect("Failed to get libclang instance");
    let index = clang::Index::new(&clang, false, true);

    let mut clang_args = clang_c_include_path_args().unwrap_or_else(|err| {
        eprintln!("Failed to get default C include paths: {err}");
        Vec::new()
    });
    clang_args.push("-xc".into());
    let translation_unit = index
        .parser(PathBuf::from(source_path))
        .arguments(&clang_args)
        .skip_function_bodies(true)
        .parse()?;

    let parsing_failed = translation_unit
        .get_diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.get_severity() >= clang::diagnostic::Severity::Error);
    if parsing_failed {
        return Err("Clang failed to parse the file provided".into());
    }

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

    for func in functions {
        println!("{func}");
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
