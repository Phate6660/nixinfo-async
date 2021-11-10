use std::error::Error as Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

/// Returns the exit code of `which getprop > /dev/null 2>&1"`
async fn exit_code() -> Result<i32, Box<dyn Error>> {
    let status = std::process::Command::new("sh")
        .args(&["-c", "which getprop > /dev/null 2>&1"])
        .status()
        .expect("");
    Ok(status.code().unwrap())
}

async fn read(file: File) -> Result<String, Box<dyn Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).await?;
    Ok(contents)
}

async fn line(file: File, line: usize) -> Result<String, Box<dyn Error>> {
    let contents = read(file).await?;
    Ok(contents.split('\n').collect::<Vec<&str>>()[line].to_string())
}

async fn get(file: File, x: usize) -> Result<String, Box<dyn Error>> {
    let line = line(file, x).await?;
    let line_vec: Vec<&str> = line.split(':').collect();
    Ok(line_vec[1].to_string())
}

async fn format(info: String) -> String {
    info.replace("(TM)", "")
        .replace("(R)", "")
        .replace("     ", " ")
}

/// Obtain CPU model, outputs to a Result<String>
pub async fn cpu() -> Result<String, Box<dyn Error>> {
    let file = File::open("/proc/cpuinfo").await?;
    async fn info(file: File, line: usize) -> Result<String, Box<dyn Error>> {
        let info = get(file, line).await?;
        Ok(format(info).await.trim().to_string().replace("\n", ""))
    }
    if exit_code().await? != 1 {
        Ok(info(file, 1).await?)
    } else {
        Ok(info(file, 4).await?)
    }
}

pub async fn dist(path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(path).await?;
    let line: String = line(file, 0).await?; // Expects NAME= to be on first line
    let distro_vec: Vec<&str> = line.split('=').collect();
    Ok(String::from(distro_vec[1]))
}
