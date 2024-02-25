use clap::{ArgGroup, Parser, Subcommand};
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ExecutionMode {
    TranslateNormal,
    TranslateInteractive,
    ListSourceLangs,
    ListTargetLangs,
    SetApiKey,
    SetDefaultTargetLang,
    DisplaySettings,
    ClearSettings,
    PrintUsage,
}

#[derive(Clone, Debug)]
pub struct ArgStruct {
    pub execution_mode: ExecutionMode,
    pub api_key: Option<String>,
    pub default_target_lang: Option<String>,
    pub translate_from: Option<String>,
    pub multilines: bool,
    pub translate_to: Option<String>,
    pub source_text: Option<String>,
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
            .args(["api_key", "target_lang", "display", "clear"]),
    ))]
    Set {
        /// Set api-key
        #[arg(short, long)]
        api_key: Option<String>,
    
        /// Set default target language
        #[arg(short, long)]
        target_lang: Option<String>,

        /// Display settings
        #[arg(short, long)]
        display: bool,
    
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

pub fn parser() -> ArgStruct {
    let args = Args::parse();
    let mut arg_struct = ArgStruct {
        execution_mode: ExecutionMode::TranslateInteractive,
        api_key: None,
        default_target_lang: None,
        translate_from: None,
        translate_to: None,
        multilines: false,
        source_text: None,
    };

    // Multilines
    if args.multilines == true {
        arg_struct.multilines = true;
    }

    // Usage
    if args.usage == true {
        arg_struct.execution_mode = ExecutionMode::PrintUsage;
        return arg_struct;
    }

    // Subcommands
    if let Some(subcommands) = args.subcommands {
        match subcommands {
            SubCommands::Set { api_key, target_lang: default_lang, display, clear } => {
                if let Some(api_key) = api_key {
                    arg_struct.execution_mode = ExecutionMode::SetApiKey;
                    arg_struct.api_key = Some(api_key);
                }
                if let Some(default_lang) = default_lang {
                    arg_struct.execution_mode = ExecutionMode::SetDefaultTargetLang;
                    arg_struct.default_target_lang = Some(default_lang);
                }
                if display == true {
                    arg_struct.execution_mode = ExecutionMode::DisplaySettings;
                }
                if clear == true {
                    arg_struct.execution_mode = ExecutionMode::ClearSettings;
                }
                return arg_struct;
            }
            SubCommands::List { source_langs, target_langs } => {
                if source_langs == true {
                    arg_struct.execution_mode = ExecutionMode::ListSourceLangs;
                }
                if target_langs == true {
                    arg_struct.execution_mode = ExecutionMode::ListTargetLangs;
                }
                return arg_struct;
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
    if let Some(source_text) = args.source_text {
        arg_struct.execution_mode = ExecutionMode::TranslateNormal;
        arg_struct.source_text = Some(source_text.join(" "));
    }
    else {
        arg_struct.execution_mode = ExecutionMode::TranslateNormal;
        arg_struct.source_text = Some(BufReader::new(io::stdin()).lines().next().unwrap().unwrap());
    }
    println!("{}", arg_struct.clone().source_text.unwrap());
    arg_struct
}
