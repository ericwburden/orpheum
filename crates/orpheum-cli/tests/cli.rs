use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn copy_expected_artifacts(project_root: &std::path::Path) {
    let scenario_file = project_root.join(".orpheum").join("scenario.json");
    let scenario: Value =
        serde_json::from_str(&fs::read_to_string(&scenario_file).expect("scenario file readable"))
            .expect("scenario json");
    let artifacts = scenario["artifacts"].as_array().expect("artifacts array");
    let repo_root = repo_root();

    for artifact in artifacts {
        let id = artifact["id"].as_str().expect("artifact id");
        let output_path = artifact["default_output_path"]
            .as_str()
            .expect("default output path");
        let source = repo_root.join("artifacts").join(format!("{id}.md"));
        let destination = project_root.join(output_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).expect("destination parent created");
        }
        fs::copy(source, destination).expect("artifact copied");
    }
}

fn copied_binary_path(destination_root: &std::path::Path) -> std::path::PathBuf {
    let source = assert_cmd::cargo::cargo_bin("orpheum");
    let file_name = source.file_name().expect("binary file name");
    let destination = destination_root.join(file_name);
    fs::copy(&source, &destination).expect("binary copied");
    destination
}

fn set_session_cli_version(project_root: &std::path::Path, version: Option<&str>) {
    let session_file = project_root.join(".orpheum").join("session.json");
    let mut session: Value =
        serde_json::from_str(&fs::read_to_string(&session_file).expect("session file readable"))
            .expect("session json");
    match version {
        Some(value) => session["last_orpheum_cli_version"] = Value::String(value.to_string()),
        None => {
            session
                .as_object_mut()
                .expect("session object")
                .remove("last_orpheum_cli_version");
        }
    }
    fs::write(
        session_file,
        serde_json::to_string_pretty(&session).expect("session json write"),
    )
    .expect("session file updated");
}

#[test]
fn scenario_list_works() {
    Command::cargo_bin("orpheum")
        .expect("binary")
        .args(["scenario", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("project-planning"));
}

#[test]
fn scenario_list_works_with_embedded_catalog_when_binary_is_outside_repo() {
    let install_root = tempdir().expect("tempdir");
    let project = tempdir().expect("tempdir");
    let binary = copied_binary_path(install_root.path());

    Command::new(binary)
        .current_dir(project.path())
        .env_remove("ORPHEUM_CATALOG")
        .args(["scenario", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"project-planning\""));
}

#[test]
fn scenario_list_works_with_explicit_catalog_from_non_catalog_directory() {
    let project = tempdir().expect("tempdir");
    let catalog_path = repo_root().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["--catalog", &catalog_path, "scenario", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"project-planning\""));
}

#[test]
fn scenario_show_returns_json() {
    Command::cargo_bin("orpheum")
        .expect("binary")
        .args(["scenario", "show", "project-planning", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"id\": \"project-planning\""));
}

#[test]
fn scenario_show_unknown_fails() {
    Command::cargo_bin("orpheum")
        .expect("binary")
        .args(["scenario", "show", "does-not-exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("SCENARIO_NOT_FOUND"));
}

#[test]
fn apply_and_status_work() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-planning",
            "--project",
            &project_path,
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"scenario_id\": \"project-planning\"",
        ));

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["status", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"current_phase\": \"business-analyst-kickoff\"",
        ));

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["prompt", "current"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Current Orpheum Prompt"));
}

#[test]
fn status_ignores_catalog_argument_because_it_is_project_local() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-planning",
            "--project",
            &project_path,
            "--json",
        ])
        .assert()
        .success();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["--catalog", "does-not-exist", "status", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"scenario_id\": \"project-planning\"",
        ));
}

#[test]
fn status_without_session_fails() {
    let project = tempdir().expect("tempdir");
    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["status"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("NO_ACTIVE_SESSION"));
}

#[test]
fn doctor_reports_missing_gitignore() {
    let project = tempdir().expect("tempdir");
    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["doctor", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"catalog_source\": \"runtime_discovery\"",
        ))
        .stdout(predicate::str::contains("GITIGNORE_MISSING"));
}

#[test]
fn init_installs_local_skill_updates_existing_gitignore_and_persists_local_config() {
    let project = tempdir().expect("tempdir");
    fs::write(project.path().join(".gitignore"), "node_modules/\n").expect("gitignore");
    let catalog_path = repo_root().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["init", "--catalog", &catalog_path, "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"gitignore_updated\": true"))
        .stdout(predicate::str::contains("\"catalog_source\": \"explicit\""));

    let skill_file = project
        .path()
        .join(".codex")
        .join("skills")
        .join("orpheum")
        .join("SKILL.md");
    assert!(skill_file.exists(), "local orpheum skill should exist");
    assert!(
        project
            .path()
            .join(".codex")
            .join("orpheum")
            .join("config.json")
            .exists(),
        "local config should exist"
    );
    assert!(
        project.path().join("ORPHEUM.md").exists(),
        "onboarding file should exist"
    );

    let gitignore = fs::read_to_string(project.path().join(".gitignore")).expect("gitignore");
    assert!(gitignore.contains(".orpheum/"));
}

#[test]
fn init_skips_gitignore_creation_when_missing_but_writes_setup_files() {
    let project = tempdir().expect("tempdir");
    let catalog_path = repo_root().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["init", "--catalog", &catalog_path, "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"gitignore_file\": null"))
        .stdout(predicate::str::contains(
            "\"project_state\": \"initialized\"",
        ));

    assert!(
        !project.path().join(".gitignore").exists(),
        "init should not create a .gitignore file"
    );
    assert!(project.path().join("ORPHEUM.md").exists());
}

#[test]
fn init_persists_catalog_from_environment_when_available() {
    let project = tempdir().expect("tempdir");
    let catalog_path = repo_root().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .env("ORPHEUM_CATALOG", &catalog_path)
        .args(["init", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"catalog_source\": \"env\""));
}

#[test]
fn init_uses_runtime_discovery_when_no_other_catalog_source_exists() {
    let project = tempdir().expect("tempdir");

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .env_remove("ORPHEUM_CATALOG")
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Catalog source: runtime_discovery",
        ));
}

#[test]
fn init_uses_embedded_catalog_when_binary_is_outside_repo() {
    let install_root = tempdir().expect("tempdir");
    let project = tempdir().expect("tempdir");
    let binary = copied_binary_path(install_root.path());

    Command::new(binary)
        .current_dir(project.path())
        .env_remove("ORPHEUM_CATALOG")
        .args(["init", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"catalog_source\": \"embedded\""))
        .stdout(predicate::str::contains("\"catalog_root\": null"));

    assert!(
        !project
            .path()
            .join(".codex")
            .join("orpheum")
            .join("config.json")
            .exists(),
        "embedded catalog should not persist an external config file"
    );
}

#[test]
fn check_run_fails_when_artifacts_missing() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();
    let catalog_path = repo_root().to_string_lossy().to_string();
    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-discovery",
            "--project",
            &project_path,
        ])
        .assert()
        .success();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["--catalog", &catalog_path, "check", "run", "--json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("CHECK_FAILED"));
}

#[test]
fn check_run_passes_with_expected_artifacts() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();
    let catalog_path = repo_root().to_string_lossy().to_string();
    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-discovery",
            "--project",
            &project_path,
        ])
        .assert()
        .success();

    copy_expected_artifacts(project.path());

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["--catalog", &catalog_path, "check", "run", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"summary\""));
}

#[test]
fn status_reflects_verified_artifacts_after_successful_check_run() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();
    let catalog_path = repo_root().to_string_lossy().to_string();
    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-discovery",
            "--project",
            &project_path,
        ])
        .assert()
        .success();

    copy_expected_artifacts(project.path());

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["--catalog", &catalog_path, "check", "run", "--json"])
        .assert()
        .success();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["status", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"verified\""));
}

#[test]
fn doctor_reports_local_config_status_and_recovery_commands() {
    let project = tempdir().expect("tempdir");
    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["doctor", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"recovery_commands\""))
        .stdout(predicate::str::contains(
            "\"catalog_source\": \"runtime_discovery\"",
        ))
        .stdout(predicate::str::contains(
            "\"message\": \"local Orpheum config file not found\"",
        ));
}

#[test]
fn status_warns_when_cli_is_newer_than_recorded_session_version() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-planning",
            "--project",
            &project_path,
            "--json",
        ])
        .assert()
        .success();

    set_session_cli_version(project.path(), Some("0.1.0"));

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["status", "--json"])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Run `orpheum update` to refresh local Orpheum guidance",
        ))
        .stdout(predicate::str::contains(
            "\"scenario_id\": \"project-planning\"",
        ));
}

#[test]
fn init_refreshes_recorded_session_cli_version_for_active_project() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();
    let catalog_path = repo_root().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-planning",
            "--project",
            &project_path,
            "--json",
        ])
        .assert()
        .success();

    set_session_cli_version(project.path(), Some("0.1.0"));

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["init", "--catalog", &catalog_path, "--json"])
        .assert()
        .success();

    let session_file = project.path().join(".orpheum").join("session.json");
    let session: Value =
        serde_json::from_str(&fs::read_to_string(&session_file).expect("session file readable"))
            .expect("session json");
    assert_eq!(
        session["last_orpheum_cli_version"].as_str(),
        Some(env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn status_warns_when_session_has_no_recorded_cli_version() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-planning",
            "--project",
            &project_path,
            "--json",
        ])
        .assert()
        .success();

    set_session_cli_version(project.path(), None);

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["status", "--json"])
        .assert()
        .success()
        .stderr(predicate::str::contains("does not record a CLI version"))
        .stdout(predicate::str::contains(
            "\"scenario_id\": \"project-planning\"",
        ));
}

#[test]
fn update_refreshes_recorded_session_cli_version_for_active_project() {
    let project = tempdir().expect("tempdir");
    let project_path = project.path().to_string_lossy().to_string();
    let catalog_path = repo_root().to_string_lossy().to_string();

    Command::cargo_bin("orpheum")
        .expect("binary")
        .args([
            "scenario",
            "apply",
            "project-planning",
            "--project",
            &project_path,
            "--json",
        ])
        .assert()
        .success();

    set_session_cli_version(project.path(), Some("0.1.0"));

    Command::cargo_bin("orpheum")
        .expect("binary")
        .current_dir(project.path())
        .args(["update", "--catalog", &catalog_path, "--json"])
        .assert()
        .success();

    let session_file = project.path().join(".orpheum").join("session.json");
    let session: Value =
        serde_json::from_str(&fs::read_to_string(&session_file).expect("session file readable"))
            .expect("session json");
    assert_eq!(
        session["last_orpheum_cli_version"].as_str(),
        Some(env!("CARGO_PKG_VERSION"))
    );
}
