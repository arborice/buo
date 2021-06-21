# TODO

- add support for custom lua scripts to handle custom bookmark imports
- these scripts should resolve to safe Rust types to be stored via serde
- benchmark zero-copy serde against possible alternatives like protobuf, rkyv
- add dependencies for parsing DATA filetypes
- prioritize loading metadata for TEXT, AUDIO, DEV, DATA, with VIDEO last
- potentially add thumbnail support
- figure out how firefox stores bookmarks
- allow customizing bookmark sources

<br/>

## MAYBE TODO

- export db as Json??
- add support for fuzzing through skim lib??
- git support is OUT OF SCOPE, but maybe allow Lua extensibility
