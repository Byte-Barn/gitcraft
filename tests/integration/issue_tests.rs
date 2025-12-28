use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use std::fs;

/**
 Integration tests for the `gitcraft` issue subcommand.

This test suite covers the following scenarios:

- `test_issue_add_bug`: Verifies that adding a "bug" issue template creates the correct file with expected content.
- `test_issue_add_multiple`: Tests the addition of multiple issue templates in one command.
- `test_issue_add_with_dir`: Ensures that an issue template can be added to a specified directory.
- `test_issue_add_force_overwrite`: Tests that an existing issue template file is not overwritten unless the `--force` flag is used.
- `test_issue_add_invalid_type`: Confirms that an unknown issue template returns an appropriate error.
- `test_issue_add_unknown_argument`: Checks that an unknown argument results in an error.
- `test_issue_add_no_template`: Ensures that running the command without specifying a template results in an error.
- `test_issue_add_valid_and_invalid_templates`: Tests adding both valid and invalid templates in one command, ensuring the valid template is still created.
- `test_issue_add_default_with_output_without_ext`: Tests adding a default issue template with an output name that does not have an extension.
- `test_issue_add_default_with_output_with_ext`: Tests adding a default issue template with an output name that has an extension.
- `test_issue_add_uneven_templates_and_outputs`: Ensures that an error is raised when the number of templates does not match the number of output file names
- `test_issue_list`: Ensures the list command displays available issue templates.
- `test_issue_preview_bug`: Validates that the preview command displays the content of the "bug" issue template.
- `test_issue_preview_multiple`: Validates that the preview command displays the content of multiple issue templates.
- `test_issue_preview_invalid_id`: Ensures that an invalid issue template name results in an error.
- `test_issue_help_command`: Validates that the help command displays usage information for the issue subcommands.

Each test uses a temporary directory to avoid side effects and leverages `assert_cmd` and `predicates` for command-line assertions.
*/
// Import utility functions
use crate::common::test_utils::{
    assert_file_contains, assert_file_exists, create_git_repo, setup_test_env,
};

// --------     ADD COMMAND TESTS     --------

#[test]
fn test_issue_add_bug() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"));
    assert_file_contains(
        &temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"),
        "Bug Report",
    );
}

#[test]
fn test_issue_add_multiple() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "bug", "feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"));
    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/feature.yml"));
}

#[test]
fn test_issue_add_with_dir() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();
    let target_dir = temp_path.join("custom_dir");
    fs::create_dir_all(&target_dir).unwrap();

    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["add", "issue", "bug", "--dir", target_dir.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    assert_file_exists(&target_dir.join("bug.yml"));
    assert_file_contains(&target_dir.join("bug.yml"), "Bug Report");
}

#[test]
fn test_issue_add_force_overwrite() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let issue_path = temp_path.join(".github/ISSUE_TEMPLATE/bug.yml");
    fs::create_dir_all(issue_path.parent().unwrap()).unwrap();
    fs::write(&issue_path, "existing content").unwrap();

    // Try to add without force (should fail)
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "bug"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Try with force flag (should succeed)
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "bug", "--force"])
        .assert()
        .success();

    assert_file_contains(&issue_path, "Bug Report");
}

#[test]
fn test_issue_add_invalid_type() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "invalid-template"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed").or(predicate::str::contains("not found")));
}

#[test]
fn test_issue_add_no_template() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No issue template specified"));
}

#[test]
fn test_issue_add_unknown_argument() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "--unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "unexpected argument '--unknown' found",
        ));
}

#[test]
fn test_issue_add_valid_and_invalid_templates() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    // Attempt to add both a valid ("bug") and invalid ("not-a-template") template in one command
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "bug", "not-a-template"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Request failed")
                .or(predicate::str::contains("not found"))
                .or(predicate::str::contains("not-a-template")),
        );

    // Ensure the valid template file ("bug") was still created
    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"));
    assert_file_contains(
        &temp_path.join(".github/ISSUE_TEMPLATE/bug.yml"),
        "Bug Report",
    );
}

#[test]
fn test_issue_add_default_with_output_without_ext() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    // Add "feature" template and specify output file without extension
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "feature", "-o", "feat"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    // Should create ".github/ISSUE_TEMPLATE/feat.yml"
    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/feat.yml"));
    assert_file_contains(
        &temp_path.join(".github/ISSUE_TEMPLATE/feat.yml"),
        "Feature Request",
    );
}

#[test]
fn test_issue_add_default_with_output_with_ext() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    // Add "feature" template and specify output file with extension
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "feature", "-o", "feat.yml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added issue template").or(predicate::str::contains("✓")));

    // Should create ".github/ISSUE_TEMPLATE/feat.yml"
    assert_file_exists(&temp_path.join(".github/ISSUE_TEMPLATE/feat.yml"));
    assert_file_contains(
        &temp_path.join(".github/ISSUE_TEMPLATE/feat.yml"),
        "Feature Request",
    );
}

#[test]
fn test_issue_add_uneven_templates_and_outputs() {
    let temp_dir = setup_test_env();
    let temp_path = temp_dir.path().to_path_buf();

    create_git_repo(&temp_path);

    // Pass two templates but only one output file
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.current_dir(&temp_path);
    cmd.args(&["add", "issue", "feature", "bug", "-o", "feat"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "The number of templates and output file names must match.",
        ));
}

// --------     LIST COMMAND TEST     --------

#[test]
fn test_issue_list() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["list", "issues"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bug"))
        .stdout(predicate::str::contains("feature"));
}

// --------     PREVIEW COMMAND TEST     --------

#[test]
fn test_issue_preview_bug() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["preview", "issue", "bug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Report"));
}

#[test]
fn test_issue_preview_multiple() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["preview", "issue", "bug", "feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bug Report"))
        .stdout(predicate::str::contains("Feature Request"));
}

#[test]
fn test_issue_preview_invalid_id() {
    let _temp_dir = setup_test_env();
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["preview", "issue", "not-a-template"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed").or(predicate::str::contains("not found")));
}

// --------     HELP COMMAND TEST     --------

#[test]
fn test_issue_help_command() {
    let _temp_dir = setup_test_env();

    // add issue help
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["add", "issue", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Add an issue template"))
        .stdout(predicate::str::contains(
            "Usage: gitcraft add issue-template",
        ))
        .stdout(predicate::str::contains("--dir"))
        .stdout(predicate::str::contains("--force"))
        .stdout(predicate::str::contains("-o, --output"));

    // list issue help
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["list", "issue", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List available issue templates"))
        .stdout(predicate::str::contains("Usage: gitcraft list issue"));

    // preview issue help
    let mut cmd = AssertCommand::cargo_bin("gitcraft").unwrap();
    cmd.args(&["preview", "issue", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Preview an issue template"))
        .stdout(predicate::str::contains("Usage: gitcraft preview issue"));
}
