# TODO

- use serde_as::serde_with HotCache for serde compat
- setup caching architecture + strategy
- ~~serialize cache for easy ffi~~

- ~~add colored output to cli for better parsing~~
- add style schema for lua (WASM?) scripts + gui bindings
- ~~do something with readme in CodeAnalyzer output extras field~~
- add support for custom lua (WASM?) scripts to handle custom bookmark imports
- these scripts should resolve to safe Rust types ~~to be serde compatible~~
- ~~benchmark zero-copy serde against possible alternatives like protobuf, rkyv~~
- ~~add dependencies for parsing DATA filetypes~~
- prioritize loading metadata for TEXT, AUDIO, DEV, ~~DATA~~ with VIDEO last
- ~~figure out how firefox stores bookmarks~~
- add cross-platform webbrowser cfg dir paths
- allow customizing bookmark sources

<br/>

## MAYBE TODO

- export db as Json??
- add support for fuzzing through skim lib??
- thumbnail support?
- git support is OUT OF SCOPE, but maybe allow Lua extensibility
