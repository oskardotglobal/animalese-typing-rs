# animalese-typing

A CLI tool which makes your keyboard play the sounds from Animal Crossing ("animalese")
when you type. Compatible with Windows, Linux (only X11) and macOS.

## Usage

```
Usage: animalese-typing [OPTIONS]

Options:
  -v, --voice <VOICE>    Voice to use (1-8), where 1-4 are "female" and 5-8 are "male" [default: 1]
  -l, --layout <LAYOUT>  [default: ansi-us] [possible values: ansi-us, iso-de]
  -h, --help             Print help
  -V, --version          Print version
```

## NixOS module

There is a NixOS module & package available in this repository.  
To install, edit your `configuration.nix`:

```nix
# configuration.nix
_: {
  imports = [ (builtins.fetchTarball "https://github.com/oskardotglobal/animalese-typing-rs/archive/mistress.tar.gz") ];

  programs.animalese-typing = {
    enable = true;

    # args passed to the CLI, see above
    voice = 1;
    layout = "ansi-us";    
  };
}
```

## Credits

Assets and original idea taken from https://github.com/joshxviii/animalese-typing
