# MarcRecordEx

MarcRecordEx is an Elixir library for working with MARC records. This is a wrapper for the Rust Library [marc_record](https://github.com/demarque/marc-record)

## Installation
### Rust
Install [Rust](https://www.rust-lang.org/tools/install)

### Installing Elixir
- install [asdf](https://github.com/asdf-vm/asdf)
- `asdf plugin add erlang`
- `asdf plugin add elixir`
- `asdf install`

### Rustler
Rustler is a library used to make [NIF](https://www.erlang.org/docs/17/tutorial/nif) out of Rust code. A NIF file allows the Erlang VM to call Rust code. It is then used to call the Rust code from Elixir.
`mix deps.get`

## Example
Start the iex console with `iex -S mix` and run the following code:
```elixir
MarcRecordEx.parse_records("./samples/marc8_multiple.mrc")
```


License
=======

`marc-record-ex` is distributed under the terms of the MIT license.
