use async_std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Test {
    pub stdin: PathBuf,
    pub stdout: PathBuf
}