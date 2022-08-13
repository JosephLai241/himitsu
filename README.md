```
            ___           _,.---,---.,_
            |         ,;~'             '~;,
            |       ,;                     ;,
   Frontal  |      ;                         ; ,--- Supraorbital Foramen
    Bone    |     ,'                         /'
            |    ,;                        /' ;,
            |    ; ;      .           . <-'  ; |
            |__  | ;   ______       ______   ;<----- Coronal Suture
           ___   |  '/~"     ~" . "~     "~\'  |
           |     |  ~  ,-~~~^~, | ,~^~~~-,  ~  |
 Maxilla,  |      |   |        }:{        | <------ Orbit
Nasal and  |      |   l       / | \       !   |
Zygomatic  |      .~  (__,.--" .^. "--.,__)  ~.
  Bones    |      |    ----;' / | \ `;-<--------- Infraorbital Foramen
           |__     \__.       \/^\/       .__/
              ___   V| \                 / |V <--- Mastoid Process
              |      | |T~\___!___!___/~T| |
              |      | |`IIII_I_I_I_IIII'| |
     Mandible |      |  \,III I I I III,/  |
              |       \   `~~~~~~~~~~'    /
              |         \   .       . <-x---- Mental Foramen
              |__         \.    ^    ./
                            ^~~~^~~~^


                              秘密

                           シークレッツ
```

> `Himitsu` (`hmu`) - An encrypted command-line tool for managing secrets on your machine.

# Table of Contents

* [Why `himitsu`?](#why-himitsu)
* [Installation](#installation)
	+ [Compile From Source](#compile-from-source)
* [Usage](#usage)
	+ [Initial Setup](#initial-setup)
	+ [Subcommands](#subcommands)
		* [`hmu add`](#hmu-add)
		* [`hmu edit`](#hmu-edit)
		* [`hmu remove`](#hmu-remove)
		* [`hmu use`](#hmu-use)

# Why `himitsu`?

> himitsu – 秘密 (ひみつ)
> > secret

Everyone has secrets they want to keep hidden from others. Software developers in particular are often exposed to many company secrets that may be detrimental to the company if used maliciously, such as API authentication tokens.

`himitsu` aims to mitigate the risk associated with storing unprotected secrets on your machine by providing a secrets management interface, allowing you to securely store/access sensitive data. Think of it like a command-line Bitwarden or LastPass.

`himitsu` additionally acts as a centralized location to store secrets on your machine, eliminating the need to traverse endless configuration files to find a particular secret.

# Installation

## Compile From Source

> **NOTE:** You will need [Rust][Rust] installed on your system to complie this program.

Clone this repository and `cd` into the `himitsu/` directory. Then run the following command:

```
cargo build --release
```

The compiled binary is located in `himitsu/target/release/` directory and is named `hmu`. You can now move the `hmu` binary to a different location for easier access, such as `/usr/local/bin` or `usr/bin`.

# Usage

## Initial Setup

On the first run, `himitsu` will ask you to setup up a password for your vault.

![Setup][setup]

## Subcommands

`himitsu` currently implements four subcommands: `add`, `edit`, `remove`, and `use`. You can print the help message for each of these subcommands by appending the `--help` flag after the subcommand.

## `hmu add`

![Add secret][add]

This subcommand allows you to add a new secret to the data store.

You can also pass in an additional positional argument and flags to this subcommand to quickly set the secret's label, category, and any tags before logging in to bypass the interactive prompts for these aspects of the secret. Ie.

```
hmu add [<SECRET_LABEL>] [-c <CATEGORY>] [-t <SPACE_DELIMITED_TAGS>]
```

> **NOTE:** Include the `-t`/`--tags` last - this flag accepts space-delimited tags and may misinterpret another flag as a tag if used before other flags or positional argument.

> **TIP:** The `[<SECRET_LABEL>]` positional argument accepts regex expressions.

## `hmu edit`

![Edit secret][edit]

This subcommand allows you to edit an existing secret in the data store.

You can also pass in an additional positional argument to search for a secret by its label. Ie.

```
hmu edit [<SECRET_LABEL>]
```

> **TIP:** The `[<SECRET_LABEL>]` positional argument accepts regex expressions.

## `hmu remove`

![Remove secret][remove]

This subcommand allows you to remove an existing secret in the data store.

You can also pass in an additional positional argument to search for a secret by its label. Ie.

```
hmu remove [<SECRET_LABEL>]
```

> **TIP:** The `[<SECRET_LABEL>]` positional argument accepts regex expressions.

## `hmu use`

![Use secret][use]

This subcommand allows you to use a secret in the data store. After authentication, the secret will be copied to your clipboard so you can quickly paste it wherever you need to use the secret.

You can also pass in an additional positional argument to search for a secret by its label. Ie.

```
hmu use [<SECRET_LABEL>]
```

> **TIP:** The `[<SECRET_LABEL>]` positional argument accepts regex expressions.

[Rust]: https://www.rust-lang.org/

<!-- DEMO GIFS -->
[add]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/add.gif
[edit]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/edit.gif
[remove]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/remove.gif
[setup]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/setup.gif
[use]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/use.gif
