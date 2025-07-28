# gltf2obj_exporter

A simple Rust command-line tool to batch convert `.gltf` models into `.obj` files.  
It parses geometry from `.gltf` and `.bin` files and exports vertex and face data.

## Features

- Parses `.gltf` files with external `.bin` buffers
- Extracts vertex positions and triangle indices
- Outputs Wavefront `.obj` format (geometry only)
- Batch processing from input folder

## Requirements

- Rust (https://www.rust-lang.org/tools/install)

## Build

Clone the repository and build with Cargo:

cargo build --release

## Usage

Prepare your assets like this:

assets/
├── model.gltf
└── model.bin

Then run:

cargo run -- --input assets --output converted

After successful conversion:

converted/
└── model.obj

## Main Convert Logic
1. Load the .gltf file
2. Read the external .bin file it refers to
3. For each mesh:
   a. Write vertex positions as "v x y z"
   b. Write triangle indices as "f a b c" (adding +1 to each index)
   c. Track total number of vertices to avoid index conflicts
4. Return the full .obj text as a string

## Notes

- Only `.gltf` files with external `.bin` are supported (not `.glb`)
- Only vertex (`v`) and face (`f`) data are exported
- Texture and material information are ignored

## License

MIT or Apache-2.0
