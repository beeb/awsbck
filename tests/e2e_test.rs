use std::{
    collections::HashMap,
    env,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use dockertest::{
    waitfor::{MessageSource, MessageWait},
    DockerTest, Source, TestBodySpecification,
};
use tokio::time::sleep;

#[test]
fn e2e_test() {
    // start S3Mock container
    let mut test = DockerTest::new().with_default_source(Source::DockerHub);
    let mut container_env = HashMap::new();
    // add default bucket "foo"
    container_env.insert("initialBuckets".to_string(), "foo".to_string());
    let mut aws = TestBodySpecification::with_repository("adobe/s3mock")
        .set_wait_for(Box::new(MessageWait {
            // wait until container has finished initializing
            message: "Started S3MockApplication".to_string(),
            source: MessageSource::Stdout,
            timeout: 10,
        }))
        .replace_env(container_env);
    // expose HTTP port
    aws.modify_port_map(9090, 9090);
    test.provide_container(aws);

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

        // run on a cron schedule
        let mut cmd = Command::new(&path)
            .env("AWSBCK_TESTING_E2E", "1")
            .args([
                "-b",
                "foo",
                "--id",
                "bar",
                "-k",
                "baz",
                "-f",
                "cron_archive",
                "-c",
                "*/1 * * * * * *",
            ])
            .arg("./src")
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");
        // wait for a few seconds
        sleep(std::time::Duration::from_secs(3)).await;
        // kill process and retrieve output on stderr
        cmd.kill().unwrap();
        let mut stderr = cmd.stderr.take().unwrap();
        let mut output = String::new();
        stderr.read_to_string(&mut output).unwrap();
        // check output
        assert!(output.contains("*/1 * * * * * *"));
        assert!(output.contains("Next backup scheduled for"));

        // check bucket contents
        env::set_var("AWS_ACCESS_KEY_ID", "bar");
        env::set_var("AWS_SECRET_ACCESS_KEY", "baz");
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region("us-east-1")
            .endpoint_url("http://127.0.0.1:9090")
            .load()
            .await;
        let client = Client::new(&shared_config);
        let resp = client.list_objects_v2().bucket("foo").send().await.unwrap();
        let objects: Vec<_> = resp
            .contents()
            .iter()
            .map(|o| o.key().unwrap_or_default())
            .collect();
        let all_sizes = resp
            .contents()
            .iter()
            .map(|o| o.size())
            .all(|s| s.unwrap_or_default() > 1000);

        assert_eq!(
            objects,
            vec![
                "awsbck_src.tar.gz",
                "cron_archive.tar.gz",
                "test/test.tar.gz"
            ]
        );
        assert!(all_sizes);
    });
}
