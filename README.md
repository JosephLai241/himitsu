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

![Crates.io](https://img.shields.io/crates/v/himitsu?logo=rust&style=for-the-badge)
![License](https://img.shields.io/github/license/JosephLai241/himitsu?style=for-the-badge)

# Table of Contents

* [Why `himitsu`?](#why-himitsu)
* [Installation](#installation)
	+ [From crates.io](#from-cratesio)
	+ [Compile From Source](#compile-from-source)
* ["How Does It Work?"](#how-does-it-work)
	+ [Password Hashing and Validation](#password-hashing-and-validation)
	+ [Encrypting and Decrypting Secrets](#encrypting-and-decrypting-secrets)
	+ ["How are secrets stored on my machine?"](#how-are-secrets-stored-on-my-machine)
		* [`crypt.json`](#cryptjson)
		* [The `closet/` Directory](#the-closet-directory)
		* [The `lookup/` Directory](#the-lookup-directory)
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

## From [crates.io][Crates.io]

> **NOTE:** You will need [Rust][Rust] installed on your system to install this program with this method.

```
cargo install himitsu
```

## Compile From Source

> **NOTE:** You will need [Rust][Rust] installed on your system to complie this program.

Clone this repository and `cd` into the `himitsu/` directory. Then run the following command:

```
cargo build --release
```

The compiled binary is located in `himitsu/target/release/` directory and is named `hmu`. You can now move the `hmu` binary to a different location for easier access, such as `/usr/local/bin` or `usr/bin`.

# How Does It Work?

## Password Hashing and Validation

`himitsu` uses [Argon2id][Argon2] for password hashing and validation (when you set up your vault's password and each time you log in).

## Encrypting and Decrypting Secrets

`himitsu` uses the [XChaCha20-Poly1305][Xchacha20] AEAD algorithm to encrypt/decrypt secrets.

## "How are secrets stored on my machine?"

This is a sample directory structure that is generated in the data directory for `himitsu`.

```
himitsu
├── closet
│   ├── 81b3f4cdd21c86843c35bea23c5c0e62650707deb619f8a424037f9c2542f386
│   │   ├── nonce
│   │   ├── salt
│   │   └── skeleton
│   ├── e8b59be73840676934b527bc13d8f6038e98477a1184e5ba1981ecb86daffdff
│   │   ├── nonce
│   │   ├── salt
│   │   └── skeleton
│   └── f22287baeec05da553474f8a480cd8799ad0824dad83f2c3008631db554d1482
│       ├── nonce
│       ├── salt
│       └── skeleton
├── crypt.json
└── lookup
    ├── nonce
    ├── salt
    └── table
```
The location of this data directory is based on the operating system you are using. See the [`ProjectDirs`'s `data_dir()` documentation][directories projectdirs data_dir documentation] to learn where it may be located on your machine.

### `crypt.json`

This file stores the Argon2id hash and salt. These two components are used for login validation.

### The `closet/` Directory

The `closet/` directory contains your encrypted secrets. Each secret (`skeleton`) is stored with its corresponding `nonce` and `salt` into a directory labeled with a SHA256 hash generated by hashing the secret's `anatomy` and the encrypted secret itself. A secret's `anatomy` contains its category, date created timestamp, label, and tags.

**Secrets are [lazily loaded][lazy loading]** - a secret is only decrypted when it is selected.

### The `lookup/` Directory

The `lookup/` directory contains a hash table (`table`) which maps a secret's `anatomy` to its corresponding SHA256 hash directory. See the section above to learn what is in an `anatomy`.

**The `table` is the only item that is decrypted once you log in**. When you select a secret to use, `himitsu` will find its SHA256 hash directory, pull the `skeleton`, `nonce`, and `salt`, decrypt the secret, and finally copy it to your clipboard.

# Usage

## Initial Setup

On the first run, `himitsu` will ask you to set up a password for your vault.

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

<!-- LINKS -->
[directories projectdirs data_dir documentation]: https://docs.rs/directories/4.0.1/directories/struct.ProjectDirs.html#method.data_dir
[lazy loading]: https://www.geeksforgeeks.org/what-is-lazy-loading/
[Argon2]: https://en.wikipedia.org/wiki/Argon2
[Crates.io]: https://crates.io/
[Rust]: https://www.rust-lang.org/
[Xchacha20]: https://en.wikipedia.org/wiki/ChaCha20-Poly1305#XChaCha20-Poly1305_-_Extended_Nonce_Variant

<!-- DEMO GIFS -->
[add]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/add.gif
[edit]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/edit.gif
[remove]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/remove.gif
[setup]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/setup.gif
[use]: https://github.com/JosephLai241/himitsu/blob/demo-gifs/gifs/use.gif
