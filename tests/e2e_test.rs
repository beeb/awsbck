use std::{collections::HashMap, env, path::PathBuf, process::Command};

use aws_sdk_s3::Client;
use dockertest::{
    waitfor::{MessageSource, MessageWait},
    Composition, DockerTest, Source,
};

#[test]
fn e2e_test() {
    // start S3Mock container
    let mut test = DockerTest::new().with_default_source(Source::DockerHub);
    let mut container_env = HashMap::new();
    // add default bucket "foo"
    container_env.insert("initialBuckets".to_string(), "foo".to_string());
    let mut aws = Composition::with_repository("adobe/s3mock")
        .with_container_name("aws")
        .with_wait_for(Box::new(MessageWait {
            // wait until container has finished initializing
            message: "Started S3MockApplication".to_string(),
            source: MessageSource::Stdout,
            timeout: 10,
        }))
        .with_env(container_env);
    // expose HTTP port
    aws.port_map(9090, 9090);
    test.add_composition(aws);

    test.run(|_ops| async move {
        let path = PathBuf::from(env!("CARGO_BIN_EXE_awsbck"));

        // upload with default archive name
        let output = Command::new(&path)
            .env("AWSBCK_TESTING_E2E", "1")
            .args(["-b", "foo", "--id", "bar", "-k", "baz"])
            .arg("./src")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Backup succeeded"));

        // upload with custom archive name (including slash)
        let output = Command::new(&path)
            .env("AWSBCK_TESTING_E2E", "1")
            .args(["-b", "foo", "--id", "bar", "-k", "baz", "-f", "test/test"])
            .arg("./src")
            .output()
            .expect("Failed to execute command");
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Backup succeeded"));

        // check bucket contents
        env::set_var("AWS_ACCESS_KEY_ID", "bar");
        env::set_var("AWS_SECRET_ACCESS_KEY", "baz");
        let shared_config = aws_config::from_env()
            .region("us-east-1")
            .endpoint_url("http://127.0.0.1:9090")
            .load()
            .await;
        let client = Client::new(&shared_config);
        let resp = client.list_objects_v2().bucket("foo").send().await.unwrap();
        let objects: Vec<_> = resp
            .contents()
            .unwrap_or_default()
            .iter()
            .map(|o| o.key().unwrap_or_default())
            .collect();
        let mut sizes_iter = resp.contents().unwrap_or_default().iter().map(|o| o.size());

        assert_eq!(objects, vec!["awsbck_src.tar.gz", "test/test.tar.gz"]);
        assert!(sizes_iter.all(|s| s > 1000));
    });
}
