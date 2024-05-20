use anyhow::Result;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> Result<()> {
    let dir_path = "./examples/output";
    let file_name = "a.txt";
    create_file(dir_path, file_name, "hello world").await?;
    let content = read_file(dir_path, file_name).await?;
    println!("{}", content);
    Ok(())
}

async fn create_file(dir_path: &str, file_name: &str, file_content: &str) -> Result<()> {
    create_dir_all(dir_path).await?;
    let file_path = format!("{dir_path}/{file_name}");
    let mut file = File::create(file_path).await?;
    file.write_all(file_content.as_bytes()).await?;
    Ok(())
}

async fn read_file(dir_path: &str, file_name: &str) -> Result<String> {
    let file_path = format!("{dir_path}/{file_name}");
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
