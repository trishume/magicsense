# MagicSense

WIP autocompletion engine in Rust designed to be cross-language yet intelligent.

The idea is to take advantage of cross-language properties like indentation, function definitions and type names to provide autocompletion guesses for any language.

I'm going to start with a basic identifier-based autocomplete with the additional feature of it discovering and indexing installed
library source/headers (i.e `/usr/local/include`) in order to also provide identifier completions for those indexed files if a significant component of the path for the library is mentioned near a word like `import`, `include`, `use` etc.

## Roadmap

- [x] Find executables in the project directory
- [ ] Search for header or source file paths in executables
- [ ] Index all found headers and source files for symbols

More stuff eventually...
