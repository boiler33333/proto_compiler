# proto-compiler
proto-compiler compiles multiple proto files into java or js files.

## Getting Started

1. put multiple .proto files in input directory.
2. execute following commands.

```
$ cd proto-compiler
$ cargo run java js

//if you want to compile to one language.
$ cargo run java
```

3. Some compiled files are generated in output directory.

## Prerequisites
This project needs to be able to execute the following commands.
- cargo
- protoc
- pbjs

## License
This project is licensed under the MIT License - see the LICENSE.md file for details