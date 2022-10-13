use async_std::task::block_on;
use async_std::path::PathBuf;
use crate::tasks::*;
use super::init;

#[test]
fn loader_test() {
    block_on(async {
        init();
        let task = load_task(PathBuf::from("./assets/task_example")).await.unwrap();
        debug!("{:?}", task);
    });
}