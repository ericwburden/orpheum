use std::fs;

use camino::Utf8Path;

use crate::catalog::OutputMode;
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::session_render::{build_active_markdown, build_prompt};
use crate::session_types::{SessionFiles, SessionManifest, SessionScenarioSnapshot, SessionState};

pub fn current_orpheum_cli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub(crate) fn session_files(project_root: &Utf8Path) -> SessionFiles {
    let control_dir = project_root.join(".orpheum");
    SessionFiles {
        active_file: control_dir.join("ACTIVE.md"),
        session_file: control_dir.join("session.json"),
        scenario_file: control_dir.join("scenario.json"),
        state_file: control_dir.join("state.json"),
        prompt_file: control_dir.join("prompts").join("current.md"),
        check_log_file: control_dir.join("logs").join("checks.json"),
        control_dir,
    }
}

pub fn read_session_files(
    project_root: &Utf8Path,
) -> Result<
    (
        SessionManifest,
        SessionScenarioSnapshot,
        SessionState,
        SessionFiles,
    ),
    OrpheumError,
> {
    let files = session_files(project_root);
    if !files.session_file.exists() {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::NoActiveSession,
            format!("no active Orpheum session found in {}", files.control_dir),
        ));
    }

    let manifest =
        serde_json::from_str::<SessionManifest>(&fs::read_to_string(&files.session_file)?)?;
    let snapshot = serde_json::from_str::<SessionScenarioSnapshot>(&fs::read_to_string(
        &files.scenario_file,
    )?)?;
    let state = serde_json::from_str::<SessionState>(&fs::read_to_string(&files.state_file)?)?;
    Ok((manifest, snapshot, state, files))
}

pub fn read_active_summary(project_root: &Utf8Path) -> Result<String, OrpheumError> {
    let (_, _, _, files) = read_session_files(project_root)?;
    Ok(fs::read_to_string(files.active_file)?)
}

pub fn refresh_state_files(
    project_root: &Utf8Path,
    snapshot: &SessionScenarioSnapshot,
    state: &SessionState,
    manifest: &SessionManifest,
) -> Result<(), OrpheumError> {
    let files = session_files(project_root);
    fs::write(&files.state_file, serde_json::to_string_pretty(state)?)?;
    let prompt = build_prompt(snapshot, state, OutputMode::Human);
    fs::write(&files.prompt_file, &prompt)?;
    fs::write(
        &files.active_file,
        build_active_markdown(manifest, snapshot, state),
    )?;
    Ok(())
}

pub fn refresh_session_cli_version(project_root: &Utf8Path) -> Result<bool, OrpheumError> {
    let files = session_files(project_root);
    if !files.session_file.exists() {
        return Ok(false);
    }

    let mut manifest =
        serde_json::from_str::<SessionManifest>(&fs::read_to_string(&files.session_file)?)?;
    let current = current_orpheum_cli_version().to_string();
    if manifest.last_orpheum_cli_version.as_deref() == Some(current.as_str()) {
        return Ok(false);
    }

    manifest.last_orpheum_cli_version = Some(current);
    fs::write(
        &files.session_file,
        serde_json::to_string_pretty(&manifest)?,
    )?;
    Ok(true)
}

pub fn cli_refresh_notice(project_root: &Utf8Path) -> Result<Option<String>, OrpheumError> {
    let files = session_files(project_root);
    if !files.session_file.exists() {
        return Ok(None);
    }

    let manifest =
        serde_json::from_str::<SessionManifest>(&fs::read_to_string(&files.session_file)?)?;
    let current = current_orpheum_cli_version();

    match manifest.last_orpheum_cli_version.as_deref() {
        Some(recorded) if version_is_newer(current, recorded) => Ok(Some(format!(
            "Orpheum CLI v{current} is newer than the active session's recorded CLI version v{recorded}. Run `orpheum update` to refresh local Orpheum guidance before continuing."
        ))),
        None => Ok(Some(
            "The active Orpheum session does not record a CLI version. Run `orpheum update` to refresh local Orpheum guidance before continuing.".into(),
        )),
        _ => Ok(None),
    }
}

fn version_is_newer(current: &str, recorded: &str) -> bool {
    compare_version_parts(current, recorded).is_gt()
}

fn compare_version_parts(left: &str, right: &str) -> std::cmp::Ordering {
    let left_parts = parse_version_parts(left);
    let right_parts = parse_version_parts(right);
    let max_len = left_parts.len().max(right_parts.len());

    for index in 0..max_len {
        let left_value = *left_parts.get(index).unwrap_or(&0);
        let right_value = *right_parts.get(index).unwrap_or(&0);
        match left_value.cmp(&right_value) {
            std::cmp::Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    std::cmp::Ordering::Equal
}

fn parse_version_parts(version: &str) -> Vec<u64> {
    version
        .split('.')
        .map(|part| {
            part.chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>()
        })
        .map(|digits| digits.parse::<u64>().unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::version_is_newer;

    #[test]
    fn compares_versions_numerically() {
        assert!(version_is_newer("0.3.0", "0.2.9"));
        assert!(version_is_newer("0.2.10", "0.2.9"));
        assert!(!version_is_newer("0.2.0", "0.2.0"));
        assert!(!version_is_newer("0.2.0", "0.2.1"));
    }
}
