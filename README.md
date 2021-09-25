# Aikot password manager

[![CI](https://github.com/iquiw/aikot/workflows/Rust/badge.svg)](https://github.com/iquiw/aikot/actions)

Aikot is a [password-store](https://www.passwordstore.org/) compatible password manager, with slightly different command line.

## External command dependencies

* GnuPG

## Commands

| Subcommand | Arguments       | Description                              |
| ---        | ---             | ---                                      |
| add        | SECRET [LENGTH] | Add new secret                           |
| browse     | SECRET          | Browse url of secret                     |
| clip       | SECRET          | Copy password to clipboard               |
| edit       | SECRET          | Edit secret by EDITOR                    |
| list       | [PATTERN]       | List secrets                             |
| pwgen      | LENGTH          | Generate passwords                       |
| show       | SECRET          | Display secret contents without password |
| version    |                 | Print the version                        |
