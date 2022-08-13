## 🚧 CURRENTLY UNDER DEVELOPMENT 🚧

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
	+ [`hmu add`](#hmu-add)
	+ [`hmu edit`](#hmu-edit)
	+ [`hmu remove`](#hmu-remove)
	+ [`hmu use`](#hmu-use)

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

`himitsu` implements four subcommands: `add`, `edit`, `remove`, and `use`.

# `hmu add`

This subcommand allows you to add a new secret to the data store.

# `hmu edit`

This subcommand allows you to edit an existing secret in the data store.

# `hmu remove`

This subcommand allows you to remove an existing secret in the data store.

# `hmu use`

This subcommand allows you to use a secret in the data store. After authentication, the secret will be copied to your clipboard so you can quickly paste it wherever you need to use the secret.

[Rust]: https://www.rust-lang.org/
