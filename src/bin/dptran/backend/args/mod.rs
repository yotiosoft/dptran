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
    PrintUsage,
    List,
    GeneralSettings,
    ApiSettings,
    CacheSettings,
    GlossarySettings,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GeneralSettingTarget {
    DefaultTargetLang,
    EditorCommand,
    ShowSettings,
    ClearSettings,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ListTargetLangs {
    SourceLangs,
    TargetLangs,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ApiSettingsTarget {
    FreeApiKey,
    ProApiKey,
    ClearFreeApiKey,
    ClearProApiKey,
    EndpointOfTranslation,
    EndpointOfUsage,
    EndpointOfLangs,
    EndpointOfGlossaries,
    EndpointOfGlossariesLangs,
    ClearEndpoints,
    ShowSettings,
    ClearSettings,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CacheSettingsTarget {
    EnableCache,
    DisableCache,
    MaxEntries,
    Clear,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GlossarySettingsTarget {
    CreateGlossary,
    DeleteGlossary,
    AddWordPairs,
    ShowGlossaries,
    ShowSupportedLanguages,
    SetDefaultGlossary,
    ClearDefaultGlossary,
}

#[derive(Clone, Debug)]
pub struct ArgStruct {
    pub execution_mode: ExecutionMode,
    pub translate_from: Option<String>,
    pub multilines: bool,
    pub remove_line_breaks: bool,
    pub no_cache: bool,
    pub translate_to: Option<String>,
    pub source_text: Option<String>,
    pub ofile_path: Option<String>,
    pub list_target_langs: Option<ListTargetLangs>,
    pub general_setting: Option<GeneralSettingsStruct>,
    pub api_setting: Option<ApiSettingsStruct>,
    pub cache_setting: Option<CacheSettingsStruct>,
    pub glossary_setting: Option<GlossarySettingsStruct>,
}

#[derive(Clone, Debug)]
pub struct GeneralSettingsStruct {
    pub setting_target: Option<GeneralSettingTarget>,
    pub default_target_lang: Option<String>,
    pub editor_command: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ApiSettingsStruct {
    pub setting_target: Option<ApiSettingsTarget>,
    pub api_key_free: Option<String>,
    pub api_key_pro: Option<String>,
    pub endpoint_of_translation: Option<String>,
    pub endpoint_of_usage: Option<String>,
    pub endpoint_of_langs: Option<String>,
    pub endpoint_of_glossaries: Option<String>,
    pub endpoint_of_glossaries_langs: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CacheSettingsStruct {
    pub setting_target: Option<CacheSettingsTarget>,
    pub max_entries: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct GlossarySettingsStruct {
    pub setting_target: Option<GlossarySettingsTarget>,
    pub target_name: Option<String>,
    pub target_id: Option<dptran::GlossaryID>,
    pub create: bool,
    pub delete: bool,
    pub add_word_pairs: Option<Vec<String>>,
    pub show_glossaries: bool,
    pub supported_languages: bool,
    pub set_default_glossary: bool,
    pub source_lang: Option<String>,
    pub target_lang: Option<String>,
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source text.
    source_text: Option<Vec<String>>,

    /// Set source language.
    /// If not specified, the source language is automatically detected.
    #[arg(short, long)]
    from: Option<String>,

    /// Set target language.
    /// If not specified, the target language is set to the default target language.
    #[arg(short, long)]
    to: Option<String>,

    /// Input multiple lines.
    #[arg(short, long)]
    multilines: bool,

    /// Remove line breaks from the input text.
    #[arg(short, long)]
    remove_line_breaks: bool,

    /// Do not cache translations.
    #[arg(long)]
    no_cache: bool,

    /// Print usage of DeepL API. (This option will be deprecated in future versions. Please use `dptran usage` instead.)
    #[arg(short, long)]
    usage: bool,

    /// Input file.
    #[arg(short, long)]
    input_file: Option<String>,

    /// Output file.
    #[arg(short, long)]
    output_file: Option<String>,

    /// Editor mode.
    /// The editor can be configured by `dptran config -e <editor_command>`
    #[arg(short, long)]
    editor: bool,

    /// subcommands
    #[clap(subcommand)]
    subcommands: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
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

    /// General settings such as default target language and editor command.
    #[command(group(
        ArgGroup::new("setting_vers")
            .required(true)
            .args(["target_lang", "editor_command", "show", "clear_all"])
    ))]
    Config {
        /// Set default target language.
        #[arg(short, long)]
        target_lang: Option<String>,

        /// Set editor command (e.g. `vi`, `vim` or `emacs -nw`).
        #[arg(short, long)]
        editor_command: Option<String>,

        /// Show settings.
        #[arg(short, long)]
        show: bool,

        /// Clear settings. This includes only the general settings. API settings are not affected.
        #[arg(short, long)]
        clear_all: bool,
    },

    /// Api settings such as API keys and endpoint URLs.
    #[command(group(
        ArgGroup::new("api_vers")
            .required(true)
            .args(["api_key_free", "api_key_pro", "clear_free_api_key", "clear_pro_api_key",
                    "endpoint_of_translation", "endpoint_of_usage", "endpoint_of_langs",
                    "clear_endpoints", "show", "clear_all"])
    ))]
    Api {
        /// Set DeepL API key (free).
        #[arg(short='f', long)]
        api_key_free: Option<String>,

        /// Set DeepL API key (pro).
        #[arg(short='p', long)]
        api_key_pro: Option<String>,

        /// Clear DeeL API key (free)
        #[arg(long)]
        clear_free_api_key: bool,

        /// Clear DeeL API key (pro)
        #[arg(long)]
        clear_pro_api_key: bool,
    
        /// Endpoint of translation API. (e.g. `https://api-free.deepl.com/v2/translate`)
        #[arg(short='t', long)]
        endpoint_of_translation: Option<String>,

        /// Endpoint of usage API. (e.g. `https://api-free.deepl.com/v2/usage`)
        #[arg(short='u', long)]
        endpoint_of_usage: Option<String>,

        /// Endpoint of languages API. (e.g. `https://api-free.deepl.com/v2/languages`)
        #[arg(short='l', long)]
        endpoint_of_langs: Option<String>,

        /// Endpoint of glossaries API. (e.g. `https://api-free.deepl.com/v2/glossaries`)
        #[arg(short='g', long)]
        endpoint_of_glossaries: Option<String>,

        /// Endpoint of glossaries language pairs API. (e.g. `https://api-free.deepl.com/v2/glossary-language-pairs`)
        #[arg(short='a', long)]
        endpoint_of_glossaries_langs: Option<String>,

        /// Clear endpoints to default values.
        #[arg(long)]
        clear_endpoints: bool,

        /// Show API settings.
        #[arg(short, long)]
        show: bool,

        /// Clear API settings. This includes API keys and endpoints. General settings are not affected.
        #[arg(short, long)]
        clear_all: bool,
    },

    /// Cache settings such as enabling/disabling cache, setting max entries, and clearing cache.
    #[command(group(
        ArgGroup::new("cache_vers")
            .required(true)
            .args(["max_entries", "clear"]),
    ))]
    Cache {
        /// Enable cache.
        #[arg(long)]
        enable_cache: bool,

        /// Disable cache.
        #[arg(long)]
        disable_cache: bool,

        /// Set cache max entries (default: 100).
        #[arg(short, long)]
        max_entries: Option<usize>,
    
        /// Clear chache.
        #[arg(short, long)]
        clear: bool,
    },

    /// Glossary settings such as creating/deleting glossaries, showing glossaries, and setting default glossary.
    #[clap(
        group(ArgGroup::new("conflict_options_for_create")
        .args(&["create"])
        .multiple(true)
        .conflicts_with_all(&["remove", "add_word_pairs", "list", "supported_languages", "set_default_glossary", 
                "clear_default_glossary"])),

        group(ArgGroup::new("conflict_options_for_remove")
        .args(&["remove"])
        .multiple(true)
        .conflicts_with_all(&["create", "add_word_pairs", "list", "supported_languages", "set_default_glossary", 
                "clear_default_glossary"])),

        group(ArgGroup::new("conflict_options_for_add_word_pairs")
        .args(&["add_word_pairs"])
        .multiple(true)
        .conflicts_with_all(&["create", "remove", "list", "supported_languages", "set_default_glossary", 
                "clear_default_glossary"])),

        group(ArgGroup::new("conflict_options_for_list")
        .args(&["list"])
        .multiple(true)
        .conflicts_with_all(&["create", "remove", "add_word_pairs", "supported_languages", "set_default_glossary", 
                "clear_default_glossary"])),

        group(ArgGroup::new("conflict_options_for_set_default_glossary")
        .args(&["set_default_glossary"])
        .multiple(true)
        .conflicts_with_all(&["create", "remove", "add_word_pairs", "list", "supported_languages", 
                "clear_default_glossary"])),

        group(ArgGroup::new("conflict_options_for_clear_default_glossary")
        .args(&["clear_default_glossary"])
        .multiple(true)
        .conflicts_with_all(&["create", "remove", "add_word_pairs", "list", "supported_languages", 
                "set_default_glossary"])),
    )]
    Glossary {
        /// A glossary that is being targeted.
        #[arg(short, long, conflicts_with = "id", requires = "create", requires = "remove", requires = "add_word_pairs", 
                requires = "set_default_glossary")]
        name: Option<String>,

        /// The ID of the glossary that is being targeted.
        #[arg(short, long, conflicts_with = "name", requires = "create", requires = "remove", requires = "add_word_pairs", 
                requires = "set_default_glossary")]
        id: Option<dptran::GlossaryID>,

        /// Create a new glossary with the targeted glossary name.
        #[arg(short, long, requires = "name", conflicts_with = "id", conflicts_with = "source_lang", conflicts_with = "target_lang")]
        create: bool,

        /// Remove the targeted glossary.
        #[arg(short, long, requires = "name", requires = "id", conflicts_with = "source_lang", conflicts_with = "target_lang")]
        remove: bool,

        /// Add word pairs to the targeted glossary.
        #[arg(short, long, num_args = 1.., requires = "name", requires = "id", requires = "source_lang", requires = "target_lang")]
        add_word_pairs: Vec<String>,

        /// Show all glossaries in the targeted glossary storage.
        #[arg(short, long, conflicts_with = "name", conflicts_with = "id")]
        list: bool,

        /// Show supported languages for glossaries.
        #[arg(short='u', long, conflicts_with = "name", conflicts_with = "id")]
        supported_languages: bool,

        /// Set the default glossary.
        #[arg(short='d', long, requires = "name", requires = "id", conflicts_with = "source_lang", conflicts_with = "target_lang")]
        set_default_glossary: bool,

        /// Clear the default glossary.
        #[arg(short='e', long, conflicts_with = "name", conflicts_with = "id", conflicts_with = "source_lang", conflicts_with = "target_lang")]
        clear_default_glossary: bool,

        /// Source language for the glossary.
        #[arg(short, long, requires = "add_word_pairs", requires = "list")]
        source_lang: Option<String>,

        /// Target language for the glossary.
        #[arg(short, long, requires = "add_word_pairs", requires = "list")]
        target_lang: Option<String>,
    },

    /// Print usage of DeepL API.
    Usage,
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
    let configure = configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    let editor = configure.get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    if let Some(editor) = editor {
        // Parse the editor command and the arguments
        // e.g., "emacs -nw" -> "emacs", "-nw"
        let mut editor_args = editor.split_whitespace();
        let editor = editor_args.next().unwrap();
        let editor_args = editor_args.collect::<Vec<&str>>().join(" ");
        // Get tmp file path
        let config_filepath = configure.get_config_file_path().map_err(|e| RuntimeError::ConfigError(e))?;
        let tmp_filepath = config_filepath.parent().unwrap().join("tmp.txt");
        // Open by the editor
        let mut child = if editor_args.len() > 0 {
            Command::new(editor).arg(editor_args).arg(tmp_filepath.to_str().unwrap()).spawn().map_err(|e| RuntimeError::EditorError(e.to_string()))?
        }
        else {
            Command::new(editor).arg(tmp_filepath.to_str().unwrap()).spawn().map_err(|e| RuntimeError::EditorError(e.to_string()))?   
        };
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
        println!("Editor is not set. Please set the editor command by `dptran config -e`.");
        println!("\t$ dptran config -e <editor_command>");
        println!("e.g.,\t\t$ dptran config -e vi");
        println!("\t..or\t$ dptran config -e vim");
        println!("\t..or\t$ dptran config -e \"emacs -nw\"");
        Err(RuntimeError::EditorError("Editor is not set.".to_string()))
    }
}

pub fn parser() -> Result<ArgStruct, RuntimeError> {
    let args = Args::parse();
    let mut arg_struct = ArgStruct {
        execution_mode: ExecutionMode::TranslateInteractive,
        translate_from: None,
        translate_to: None,
        multilines: false,
        remove_line_breaks: false,
        no_cache: false,
        source_text: None,
        ofile_path: None,
        list_target_langs: None,
        general_setting: Some(GeneralSettingsStruct {
            setting_target: None,
            default_target_lang: None,
            editor_command: None,
        }),
        api_setting: Some(ApiSettingsStruct {
            setting_target: None,
            api_key_free: None,
            api_key_pro: None,
            endpoint_of_translation: None,
            endpoint_of_usage: None,
            endpoint_of_langs: None,
            endpoint_of_glossaries: None,
            endpoint_of_glossaries_langs: None,
        }),
        cache_setting: Some(CacheSettingsStruct {
            setting_target: None,
            max_entries: None,
        }),
        glossary_setting: Some(GlossarySettingsStruct {
            setting_target: None,
            target_name: None,
            target_id: None,
            create: false,
            delete: false,
            add_word_pairs: None,
            show_glossaries: false,
            supported_languages: false,
            set_default_glossary: false,
            source_lang: None,
            target_lang: None,
        }),
    };

    // Multilines
    if args.multilines == true {
        arg_struct.multilines = true;
    }

    // Remove line breaks
    if args.remove_line_breaks == true {
        arg_struct.remove_line_breaks = true;
    }

    // No cache
    if args.no_cache == true {
        arg_struct.no_cache = true;
    }

    // Usage
    if args.usage == true {
        println!("----------------------------------------------------------
Caution: This option will be deprecated in future versions. 
Please use the subcommand `dptran usage` instead.
----------------------------------------------------------");

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
            SubCommands::Config { target_lang: default_lang,  
                    editor_command, show, clear_all } => {
                if let Some(default_lang) = default_lang {
                    arg_struct.execution_mode = ExecutionMode::GeneralSettings;
                    arg_struct.general_setting.as_mut().unwrap().setting_target = Some(GeneralSettingTarget::DefaultTargetLang);
                    arg_struct.general_setting.as_mut().unwrap().default_target_lang = Some(default_lang);
                }
                if let Some(editor_command) = editor_command {
                    arg_struct.execution_mode = ExecutionMode::GeneralSettings;
                    arg_struct.general_setting.as_mut().unwrap().setting_target = Some(GeneralSettingTarget::EditorCommand);
                    arg_struct.general_setting.as_mut().unwrap().editor_command = Some(editor_command);
                }
                if show == true {
                    arg_struct.execution_mode = ExecutionMode::GeneralSettings;
                    arg_struct.general_setting.as_mut().unwrap().setting_target = Some(GeneralSettingTarget::ShowSettings);
                }
                if clear_all == true {
                    arg_struct.execution_mode = ExecutionMode::GeneralSettings;
                    arg_struct.general_setting.as_mut().unwrap().setting_target = Some(GeneralSettingTarget::ClearSettings);
                }
                return Ok(arg_struct);
            }
            SubCommands::List { source_langs, target_langs } => {
                if source_langs == true {
                    arg_struct.execution_mode = ExecutionMode::List;
                    arg_struct.list_target_langs = Some(ListTargetLangs::SourceLangs);
                }
                if target_langs == true {
                    arg_struct.execution_mode = ExecutionMode::List;
                    arg_struct.list_target_langs = Some(ListTargetLangs::TargetLangs);
                }
                return Ok(arg_struct);
            }
            SubCommands::Api { api_key_free, api_key_pro, clear_free_api_key, clear_pro_api_key,
                    endpoint_of_translation, endpoint_of_usage, endpoint_of_langs , 
                    endpoint_of_glossaries, endpoint_of_glossaries_langs,
                    clear_endpoints, show, clear_all } => {
                if let Some(api_key) = api_key_free {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::FreeApiKey);
                    arg_struct.api_setting.as_mut().unwrap().api_key_free = Some(api_key);
                }
                if let Some(api_key) = api_key_pro {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::ProApiKey);
                    arg_struct.api_setting.as_mut().unwrap().api_key_pro = Some(api_key);
                }
                if clear_free_api_key == true {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::ClearFreeApiKey);
                    arg_struct.api_setting.as_mut().unwrap().api_key_free = None;
                }
                if clear_pro_api_key == true {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::ClearProApiKey);
                    arg_struct.api_setting.as_mut().unwrap().api_key_pro = None;
                }
                if let Some(endpoint_of_translation) = endpoint_of_translation {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::EndpointOfTranslation);
                    arg_struct.api_setting.as_mut().unwrap().endpoint_of_translation = Some(endpoint_of_translation);
                }
                if let Some(endpoint_of_usage) = endpoint_of_usage {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::EndpointOfUsage);
                    arg_struct.api_setting.as_mut().unwrap().endpoint_of_usage = Some(endpoint_of_usage);
                }
                if let Some(endpoint_of_langs) = endpoint_of_langs {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::EndpointOfLangs);
                    arg_struct.api_setting.as_mut().unwrap().endpoint_of_langs = Some(endpoint_of_langs);
                }
                if let Some(endpoint_of_glossaries) = endpoint_of_glossaries {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::EndpointOfGlossaries);
                    arg_struct.api_setting.as_mut().unwrap().endpoint_of_glossaries = Some(endpoint_of_glossaries);
                }
                if let Some(endpoint_of_glossaries_langs) = endpoint_of_glossaries_langs {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::EndpointOfGlossariesLangs);
                    arg_struct.api_setting.as_mut().unwrap().endpoint_of_glossaries_langs = Some(endpoint_of_glossaries_langs);
                }
                if clear_endpoints == true {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::ClearEndpoints);
                }
                if show == true {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::ShowSettings);
                }
                if clear_all == true {
                    arg_struct.execution_mode = ExecutionMode::ApiSettings;
                    arg_struct.api_setting.as_mut().unwrap().setting_target = Some(ApiSettingsTarget::ClearSettings);
                }
                return Ok(arg_struct);
            }
            SubCommands::Cache { enable_cache, disable_cache, max_entries, clear } => {
                if enable_cache == true {
                    arg_struct.execution_mode = ExecutionMode::CacheSettings;
                    arg_struct.cache_setting = Some(CacheSettingsStruct {
                        setting_target: Some(CacheSettingsTarget::EnableCache),
                        max_entries: None,
                    });
                }
                if disable_cache == true {
                    arg_struct.execution_mode = ExecutionMode::CacheSettings;
                    arg_struct.cache_setting = Some(CacheSettingsStruct {
                        setting_target: Some(CacheSettingsTarget::DisableCache),
                        max_entries: None,
                    });
                }
                if let Some(max_entries) = max_entries {
                    arg_struct.execution_mode = ExecutionMode::CacheSettings;
                    arg_struct.cache_setting = Some(CacheSettingsStruct {
                        setting_target: Some(CacheSettingsTarget::MaxEntries),
                        max_entries: Some(max_entries),
                    });
                }
                if clear == true {
                    arg_struct.execution_mode = ExecutionMode::CacheSettings;
                    arg_struct.cache_setting = Some(CacheSettingsStruct {
                        setting_target: Some(CacheSettingsTarget::Clear),
                        max_entries: None,
                    });
                }
                return Ok(arg_struct);
            }
            SubCommands::Glossary { name, id, create, remove, add_word_pairs,
                    list, supported_languages, set_default_glossary, clear_default_glossary,
                    source_lang, target_lang } => {
                arg_struct.execution_mode = ExecutionMode::GlossarySettings;
                if let Some(name) = name {
                    arg_struct.glossary_setting.as_mut().unwrap().target_name = Some(name);
                }
                if let Some(id) = id {
                    arg_struct.glossary_setting.as_mut().unwrap().target_id = Some(id);
                }
                if create == true {
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::CreateGlossary);
                    arg_struct.glossary_setting.as_mut().unwrap().create = true;
                }
                if remove == true {
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::DeleteGlossary);
                    arg_struct.glossary_setting.as_mut().unwrap().delete = true;
                }
                if add_word_pairs.len() > 0 {
                    if add_word_pairs.len() % 2 != 0 {
                        return Err(RuntimeError::ArgError("The number of word pairs to add must be even.".to_string()));
                    }
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::AddWordPairs);
                    arg_struct.glossary_setting.as_mut().unwrap().add_word_pairs = Some(add_word_pairs);
                }
                if list == true {
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::ShowGlossaries);
                    arg_struct.glossary_setting.as_mut().unwrap().show_glossaries = true;
                }
                if supported_languages == true {
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::ShowSupportedLanguages);
                    arg_struct.glossary_setting.as_mut().unwrap().supported_languages = true;
                }
                if set_default_glossary == true {
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::SetDefaultGlossary);
                    arg_struct.glossary_setting.as_mut().unwrap().set_default_glossary = true;
                }
                if clear_default_glossary == true {
                    arg_struct.glossary_setting.as_mut().unwrap().setting_target = Some(GlossarySettingsTarget::ClearDefaultGlossary);
                }
                if let Some(source_lang) = source_lang {
                    arg_struct.glossary_setting.as_mut().unwrap().source_lang = Some(source_lang);
                }
                if let Some(target_lang) = target_lang {
                    arg_struct.glossary_setting.as_mut().unwrap().target_lang = Some(target_lang);
                }
                return Ok(arg_struct);
            }
            SubCommands::Usage => {
                arg_struct.execution_mode = ExecutionMode::PrintUsage;
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

#[cfg(test)]
include!("tests.rs");
