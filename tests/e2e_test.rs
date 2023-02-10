use std::{collections::HashMap, path::PathBuf, process::Command};

use dockertest::{
    waitfor::{MessageSource, MessageWait},
    Composition, DockerTest, Source,
};

#[test]
fn e2e_test() {
    let mut test = DockerTest::new().with_default_source(Source::DockerHub);
    let mut container_env = HashMap::new();
    container_env.insert("initialBuckets".to_string(), "foo".to_string());
    let mut aws = Composition::with_repository("adobe/s3mock")
        .with_container_name("aws")
        .with_wait_for(Box::new(MessageWait {
            message: "Started S3MockApplication".to_string(),
            source: MessageSource::Stdout,
            timeout: 10,
        }))
        .with_env(container_env);
    aws.port_map(9090, 9090).port_map(9191, 9191);
    test.add_composition(aws);

    test.run(|ops| async move {
        let _container = ops.handle("aws");
        let path = PathBuf::from(env!("CARGO_BIN_EXE_awsbck"));
        let output = Command::new(path)
            .env("AWSBCK_TESTING_E2E", "1")
            .args(["-b", "foo", "--id", "bar", "-k", "baz"])
            .arg("./src")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Backup succeeded"));
    });
}
