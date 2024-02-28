use codespan::FileId;
use codespan::Span;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use move_model::model::{GlobalEnv, Parameter};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::{fs::OpenOptions, path::Path};
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct LintConfig {
    pub statement_count: usize,
    pub usage_frequency: usize,
}

impl Default for LintConfig {
    fn default() -> Self {
        LintConfig {
            statement_count: 10,
            usage_frequency: 2,
        }
    }
}

pub fn add_diagnostic_and_emit(
    loc: &move_model::model::Loc,
    message: &str,
    severity: codespan_reporting::diagnostic::Severity,
    env: &GlobalEnv,
    diags: &mut Vec<Diagnostic<FileId>>,
) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let label = Label::primary(
        loc.file_id(),
        loc.span().start().to_usize()..loc.span().end().to_usize(),
    )
    .with_message(message.to_string());

    let diagnostic = Diagnostic::new(severity)
        .with_message(message)
        .with_labels(vec![label]);

    diags.push(diagnostic.clone());
    emit(
        &mut writer.lock(),
        &config,
        &env.get_source_files(),
        &diagnostic,
    )
    .expect("emit must not fail");
}

pub fn add_diagnostic_and_emit_by_span(
    start: Span,
    end: Span,
    file_id: FileId,
    message: &str,
    severity: codespan_reporting::diagnostic::Severity,
    env: &GlobalEnv,
    diags: &mut Vec<Diagnostic<FileId>>,
) {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let label = Label::primary(file_id, start.start().to_usize()..end.end().to_usize())
        .with_message(message.to_string());

    let diagnostic = Diagnostic::new(severity)
        .with_message(message)
        .with_labels(vec![label]);
    diags.push(diagnostic.clone());

    emit(
        &mut writer.lock(),
        &config,
        &env.get_source_files(),
        &diagnostic,
    )
    .expect("emit must not fail");
}

pub fn get_var_info_from_func_param(index: usize, params: &[Parameter]) -> Option<&Parameter> {
    params.get(index)
}
pub fn read_config(path: &Path) -> Result<LintConfig, Box<dyn std::error::Error>> {
    eprintln!("Reading config from path: {:?}", path);
    let binding = path.join("lint.toml");
    let exist_path = Path::new(&binding);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(exist_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    if content.is_empty() {
        let default_config = LintConfig::default();
        let default_content = toml::to_string(&default_config)?;
        file.write_all(default_content.as_bytes())?;
        file.flush()?;
        content = default_content.to_string();
    }
    let config: LintConfig = toml::from_str(&content)?;

    Ok(config)
}
