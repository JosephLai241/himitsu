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

> An encrypted command-line tool for managing secrets on your machine.

# Table of Contents

* [Why `himitsu`?](#why-himitsu)
* [How Does It Work?](#how-does-it-work)
* [Usage](#usage)
	+ [`himitsu add`](#himitsu-add)
	+ [`himitsu edit`](#himitsu-edit)
	+ [`himitsu remove`](#himitsu-remove)
	+ [`himitsu use`](#himitsu-use)

# Why `himitsu`?

> himitsu – 秘密 (ひみつ)
> > secret

Everyone has secrets they want to keep hidden from others. Software developers in particular are often exposed to many company secrets that may be detrimental to the company if used maliciously, such as API authentication tokens.

`himitsu` aims to mitigate the risk associated with storing unprotected secrets on your machine by providing a secrets management interface, allowing you to securely store/access sensitive data. Think of it like a command-line Bitwarden or LastPass.

# Usage

`himitsu` implements four subcommands: `add`, `edit`, `remove`, and `use`.

# `himitsu add`

This subcommand allows you to add a new secret to the data store.

# `himitsu edit`

This subcommand allows you to edit an existing secret in the data store.

# `himitsu remove`

This subcommand allows you to remove an existing secret in the data store.

# `himitsu use`

This subcommand allows you to use a secret in the data store. After authentication, the secret will be copied to your clipboard so you can quickly paste it wherever you need to use the secret.
