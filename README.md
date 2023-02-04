# dptran

A tool to run DeepL translations on your command line.   
It's written in Rust, and uses curl to connect to the DeepL API.  
To use, you need to get DeepL API key.

## How to install?

### for Windows

1. Download the latest version from Releases.
1. Add the folder that contains dptran.exe to the PATH on Windows.

### for macOS

```bash
$ brew tap YotioSoft/dptran
$ brew install dptran
```

## How to use?

### Setting API key

Please be sure to get your DeepL API key (it's free!) and set it up on dptran before using the service.

```bash
$ dptran -c key [API key]
```

### Translate in normal mode

```bash
$ dptran Hello
こんにちは
$ dptran -t FR Hello
Bonjour
```

You can specify the source language with the ``-f`` option and the target language with the ``-t`` option.  
If you omit the ``-destination language`` option, the translation will be done in Japanese.  

For more information about language codes, see the language list:  

```bash
$ dptran -ls    # for the list of source languages
$ dptran -lt    # for the list of target languages
```

### Translate in interactive mode

```bash
$ dptran
> Hello
こんにちは
> Ich stehe jeden Tag um 7 Uhr auf.
毎日7時に起きています。
> Seriously, Hiro, you need to improve your English.
マジでヒロさん、英語力アップしてください。
> 今天玩儿得真开心！
今日は素晴らしい時間を過ごせました
> quit
```

Multiple source texts can be translated interactively.  
Exit with ``quit``.

### Translate from pipeline

You can translate the execution result of other commands.  

ex: Translate man page content into Japanese  

```bash
$ man ls | col -b | dptran
```

### Show help

For more information about commands, see help:  

```bash
$ dptran -h
```

### Displays the number of characters remaining to be translated

```bash
$ dptran -u
usage: 64785 / 500000
remaining: 435215
```

You can view the number of remaining characters that can be translated by DeepL API.  
DeepL API's free plan allows you to translate up to 500,000 characters per month.

## Change default target language

It is set to Japanese (JA) by default.  
You can change it with ``-c default-lang``.  
For example, to change it to English (EN), do the following:

```bash
$ dptran -c default-lang EN
```

## Reset settings

You can reset all settings.  
Note: The API key will be reset as well. If you wish to use dptran again, please set the API key again.  

```bash
$ dptran -c clear
```



## How to uninstall?

### for Windows

1. Remove dptran.exe
1. Remove the filepath from the PATH on Windows

### for macOS

```bash
$ brew uninstall dptran
```

After executing the above command, reboot the terminal to complete uninstallation.
