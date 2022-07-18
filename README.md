     __  __ __  ____ __     ____ ______   ___   __  __  __ 
    (( \ || // ||    ||    ||    | || |  // \\  ||\ || (( \
     \\  ||<<  ||==  ||    ||==    ||   ((   )) ||\\||  \\ 
    \_)) || \\ ||___ ||__| ||___   ||    \\_//  || \|| \_))

> An encrypted command-line tool for managing secrets on your machine.

# Table of Contents

* [Why `skeletons`?](#why-skeletons)
* [How Does It Work?](#how-does-it-work)
* [Usage](#usage)
	+ [`skeleton add`](#skeleton-add)
	+ [`skeleton edit`](#skeleton-edit)
	+ [`skeleton remove`](#skeleton-remove)
	+ [`skeleton use`](#skeleton-use)

# Why `skeletons`?

> Define: skeletons in the closet
>
> > A discreditable or embarrassing fact that someone wishes to keep secret.
>
> [source][skeletons in the closet definition]

Everyone has secrets they want to keep hidden from others. Software developers in particular are often exposed to many company secrets that may be detrimental to the company if used maliciously, such as API authentication tokens.

`skeletons` aims to mitigate the risk associated with storing unprotected secrets on your machine by providing a secrets management interface, allowing you to securely store/access sensitive data. Think of it like a command-line Bitwarden or LastPass.

I am sure there are already programs like this out there, but I wanted to experiment with local encryption by implementing my own program.

# How Does It Work?

Secrets are encrypted with the [XChaCha20-Poly1305 AEAD algorithm][XChaCha20 Wikipedia] and are unlocked with a master password hashed using [Argon2][Argon2 Wikipedia], specifically the Argon2id variant.

# Usage

`skeletons` implements four subcommands: `add`, `edit`, `remove`, and `use`.

# `skeleton add`

This subcommand allows you to add a new secret to the data store.

# `skeleton edit`

This subcommand allows you to edit an existing secret in the data store.

# `skeleton remove`

This subcommand allows you to remove an existing secret in the data store.

# `skeleton use`

This subcommand allows you to use a secret in the data store. After authentication, the secret will be copied to your clipboard so you can quickly paste it wherever you need to use the secret.

[skeletons in the closet definition]: https://www.google.com/search?client=firefox-b-1-d&q=define+skeletons+in+the+closet
[Argon2 Wikipedia]: https://en.wikipedia.org/wiki/Argon2
[XChaCha20 Wikipedia]: https://en.wikipedia.org/wiki/ChaCha20-Poly1305#XChaCha20-Poly1305_-_Extended_Nonce_Variant
