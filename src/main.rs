use clap::Parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path};
use walkdir::WalkDir;
use gltf::{self, buffer::Source};

/// Batch convert .gltf to .obj
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    fs::create_dir_all(&args.output)?;

    for entry in WalkDir::new(&args.input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "gltf").unwrap_or(false))
    {
        let path = entry.path();
        println!("Processing: {:?}", path);
        
        let obj_str = convert_gltf_to_obj(path)?;
        let output_path = Path::new(&args.output)
            .join(path.file_stem().unwrap())
            .with_extension("obj");
        let mut file = File::create(output_path)?;
        file.write_all(obj_str.as_bytes())?;
    }

    println!("Conversion complete.");
    Ok(())
}

fn convert_gltf_to_obj(path: &Path) -> anyhow::Result<String> {
    let gltf_json = std::fs::read(path)?;
    let parent_dir = path.parent().unwrap();

    // Parse the glTF JSON and manually load binary buffer
    let doc = gltf::Gltf::from_slice(&gltf_json)?;
    let mut buffers_data = Vec::new();

    for buffer in doc.buffers() {
        match buffer.source() {
            Source::Uri(uri) => {
                let bin_path = parent_dir.join(uri);
                let data = std::fs::read(bin_path)?;
                buffers_data.push(data);
            }
            Source::Bin => {
                anyhow::bail!("GLB with embedded bin not supported");
            }
        }
    }

    let buffer_data = &buffers_data[0];
    let mut obj_data = String::new();
    let mut vertex_offset = 1;

    for mesh in doc.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|_| Some(buffer_data.as_slice()));

            if let Some(positions) = reader.read_positions() {
                for pos in positions {
                    obj_data += &format!("v {} {} {}\n", pos[0], pos[1], pos[2]);
                }
            }

            if let Some(indices) = reader.read_indices().map(|i| i.into_u32()) {
                let idx: Vec<u32> = indices.collect();
                for face in idx.chunks(3) {
                    if face.len() == 3 {
                        obj_data += &format!(
                            "f {} {} {}\n",
                            face[0] + vertex_offset,
                            face[1] + vertex_offset,
                            face[2] + vertex_offset
                        );
                    }
                }
                vertex_offset += idx.len() as u32;
            }
        }
    }

    Ok(obj_data)
}
