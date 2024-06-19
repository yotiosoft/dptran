use clap::{ArgGroup, Parser, Subcommand};
use std::io::{self, Read};
use atty::Stream;
use super::RuntimeError;
use std::process::Command;
use super::configure;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ExecutionMode {
    TranslateNormal,
    TranslateInteractive,
    ListSourceLangs,
    ListTargetLangs,
    SetApiKey,
    SetDefaultTargetLang,
    SetCacheMaxEntries,
    SetEditor,
    DisplaySettings,
    ClearSettings,
    PrintUsage,
}

#[derive(Clone, Debug)]
pub struct ArgStruct {
    pub execution_mode: ExecutionMode,
    pub api_key: Option<String>,
    pub default_target_lang: Option<String>,
    pub cache_max_entries: Option<usize>,
    pub editor_command: Option<String>,
    pub translate_from: Option<String>,
    pub multilines: bool,
    pub translate_to: Option<String>,
    pub source_text: Option<String>,
    pub ofile_path: Option<String>,
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source text
    source_text: Option<Vec<String>>,

    /// Set source language
    #[arg(short, long)]
    from: Option<String>,

    /// Set target language
    #[arg(short, long)]
    to: Option<String>,

    /// Input multiple lines
    #[arg(short, long)]
    multilines: bool,

    /// Print usage of DeepL API
    #[arg(short, long)]
    usage: bool,

    /// Input file
    #[arg(short, long)]
    input_file: Option<String>,

    /// Output file
    #[arg(short, long)]
    output_file: Option<String>,

    /// Editor mode
    #[arg(short, long)]
    editor: bool,

    /// subcommands
    #[clap(subcommand)]
    subcommands: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Settings
    #[command(group(
        ArgGroup::new("setting_vers")
            .required(true)
            .args(["api_key", "target_lang", "cache_max_entries", "editor_command", "show", "clear"]),
    ))]
    Set {
        /// Set api-key
        #[arg(short, long)]
        api_key: Option<String>,
    
        /// Set default target language
        #[arg(short, long)]
        target_lang: Option<String>,

        /// Set cache max entries (default: 100)
        #[arg(long)]
        cache_max_entries: Option<usize>,

        /// Set editor command (e.g. `vi`, `vim` or `emacs -nw`)
        #[arg(short, long)]
        editor_command: Option<String>,

        /// Show settings
        #[arg(short, long)]
        show: bool,
    
        /// Clear settings
        #[arg(short, long)]
        clear: bool,
    },

    /// Show list of supperted languages
    #[command(group(
        ArgGroup::new("list_vers")
            .required(true)
            .args(["source_langs", "target_langs"]),
    ))]
    List {
        /// List source languages
        #[arg(short, long)]
        source_langs: bool,
    
        /// List target languages
        #[arg(short, long)]
        target_langs: bool,
    },
}

fn load_stdin() -> io::Result<Option<String>> {
    if atty::is(Stream::Stdin) {
        return Ok(None);
    }
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(Some(buffer))
}

fn read_from_editor() -> Result<String, RuntimeError> {
    // Get editor command
    let editor = configure::get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    if let Some(editor) = editor {
        // Parse the editor command and the arguments
        // e.g., "emacs -nw" -> "emacs", "-nw"
        let mut editor_args = editor.split_whitespace();
        let editor = editor_args.next().unwrap();
        let editor_args = editor_args.collect::<Vec<&str>>().join(" ");
        // Get tmp file path
        let config_filepath = configure::get_config_file_path().map_err(|e| RuntimeError::ConfigError(e))?;
        let tmp_filepath = config_filepath.parent().unwrap().join("tmp.txt");
        // Open by the editor
        let mut child = Command::new(editor).arg(editor_args).arg(tmp_filepath.to_str().unwrap()).spawn().map_err(|e| RuntimeError::EditorError(e.to_string()))?;
        let status = child.wait().map_err(|e| RuntimeError::EditorError(e.to_string()))?;
        if !status.success() {
            return Err(RuntimeError::EditorError("Editor failed".to_string()));
        }
        // Read from the tmp file
        let text = std::fs::read_to_string(&tmp_filepath).map_err(|e| RuntimeError::FileIoError(e.to_string()))?;
        // Remove the tmp file
        std::fs::remove_file(&tmp_filepath).map_err(|e| RuntimeError::FileIoError(e.to_string()))?;
        Ok(text)
    }
    else {
        println!("Editor is not set. Please set the editor command by `dptran set -e`.");
        println!("\t$ dptran set -e <editor_command>");
        println!("e.g.,\t\t$ dptran set -e vi");
        println!("\t..or\t$ dptran set -e vim");
        println!("\t..or\t$ dptran set -e \"emacs -nw\"");
        Err(RuntimeError::EditorError("Editor is not set.".to_string()))
    }
}

pub fn parser() -> Result<ArgStruct, RuntimeError> {
    let args = Args::parse();
    let mut arg_struct = ArgStruct {
        execution_mode: ExecutionMode::TranslateInteractive,
        api_key: None,
        default_target_lang: None,
        cache_max_entries: None,
        editor_command: None,
        translate_from: None,
        translate_to: None,
        multilines: false,
        source_text: None,
        ofile_path: None,
    };

    // Multilines
    if args.multilines == true {
        arg_struct.multilines = true;
    }

    // Usage
    if args.usage == true {
        arg_struct.execution_mode = ExecutionMode::PrintUsage;
        return Ok(arg_struct);
    }

    // Output file
    if let Some(ofile_path) = args.output_file {
        arg_struct.ofile_path = Some(ofile_path);
    }

    // Subcommands
    if let Some(subcommands) = args.subcommands {
        match subcommands {
            SubCommands::Set { api_key, target_lang: default_lang, cache_max_entries, editor_command, show, clear } => {
                if let Some(api_key) = api_key {
                    arg_struct.execution_mode = ExecutionMode::SetApiKey;
                    arg_struct.api_key = Some(api_key);
                }
                if let Some(default_lang) = default_lang {
                    arg_struct.execution_mode = ExecutionMode::SetDefaultTargetLang;
                    arg_struct.default_target_lang = Some(default_lang);
                }
                if let Some(cache_max_entries) = cache_max_entries {
                    arg_struct.execution_mode = ExecutionMode::SetCacheMaxEntries;
                    arg_struct.cache_max_entries = Some(cache_max_entries);
                }
                if let Some(editor_command) = editor_command {
                    arg_struct.execution_mode = ExecutionMode::SetEditor;
                    arg_struct.editor_command = Some(editor_command);
                }
                if show == true {
                    arg_struct.execution_mode = ExecutionMode::DisplaySettings;
                }
                if clear == true {
                    arg_struct.execution_mode = ExecutionMode::ClearSettings;
                }
                return Ok(arg_struct);
            }
            SubCommands::List { source_langs, target_langs } => {
                if source_langs == true {
                    arg_struct.execution_mode = ExecutionMode::ListSourceLangs;
                }
                if target_langs == true {
                    arg_struct.execution_mode = ExecutionMode::ListTargetLangs;
                }
                return Ok(arg_struct);
            }
        }
    }

    // Translation mode (normal mode)
    if let Some(from) = args.from {
        arg_struct.translate_from = Some(from);
    }
    if let Some(to) = args.to {
        arg_struct.translate_to = Some(to);
    }
    // If input file is specified, read from the file
    if let Some(filepath) = args.input_file {
        arg_struct.execution_mode = ExecutionMode::TranslateNormal;
        arg_struct.source_text = Some(std::fs::read_to_string(&filepath).map_err(|e| RuntimeError::FileIoError(e.to_string()))?);
    }
    // If editor mode is specified, read from stdin
    else if args.editor == true {
        arg_struct.execution_mode = ExecutionMode::TranslateNormal;
        arg_struct.source_text = Some(read_from_editor()?);
    }
    // If source_text is specified, get source_text
    else if let Some(source_text) = args.source_text {
        arg_struct.source_text = Some(source_text.join(" "));
        arg_struct.execution_mode = ExecutionMode::TranslateNormal;
    }
    // If input file is not specified and args.source_text is None, try to read from stdin
    else {
        let line = load_stdin().map_err(|e| RuntimeError::StdIoError(e.to_string()))?;
        match line {
            Some(s) => {
                arg_struct.execution_mode = ExecutionMode::TranslateNormal;
                arg_struct.source_text = Some(s);
            },
            None => {
                arg_struct.execution_mode = ExecutionMode::TranslateInteractive;
                arg_struct.source_text = None;
            },
        };
    }
    Ok(arg_struct)
}
