use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let project_dir = current_dir
        .parent()
        .ok_or("Impossible not to have the parent directory")?;

    let proto_file = project_dir.join("proto/chat.proto");
    tonic_build::configure()
        .out_dir("chat_rpc")
        .compile(&[proto_file], &[project_dir])?;
    Ok(())
}
