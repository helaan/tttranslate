# TTTranslate
## Translation Tools for TTT2

TTT2 stores its translation files in lua files, which cannot be directly accepted by translation tools like [Weblate](https://weblate.org).
This repository provides tools that can convert between a simple JSON format, which is accepted by Weblate, and the Lua files as stored in the TTT2 repository.


### Json2Lua

Parses a lua file to use as a template, then reads in a JSON file containing translations. This tool is written in [Rust](https://www.rust-lang.org) and uses [Pest](https://pest.rs) as a parser.

### Lua2Json

Defines a small version of TTT2's language code, together with an ordered hash table to create a JSON file containing localizations. This tool is written in Lua and uses [json.lua](https://github.com/rxi/json.lua) and [Rici Lake's Ordered Table](http://lua-users.org/wiki/OrderedTable).
