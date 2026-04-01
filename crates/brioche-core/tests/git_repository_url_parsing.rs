use brioche_core::project::analyze::GitRefOptions;

#[test]
fn parses_ssh_url_with_colon_path_like_scp() {
    let options: GitRefOptions = serde_json::from_value(serde_json::json!({
        "repository": "ssh://git@servername:path/repo.git",
        "ref": "main",
    }))
    .expect("should normalize ssh://user@host:path URLs");

    assert_eq!(
        options.repository.as_str(),
        "ssh://git@servername/path/repo.git"
    );
}

#[test]
fn parses_scp_like_ssh_url() {
    let options: GitRefOptions = serde_json::from_value(serde_json::json!({
        "repository": "git@servername:path/repo.git",
        "ref": "main",
    }))
    .expect("should normalize scp-like git URLs");

    assert_eq!(
        options.repository.as_str(),
        "ssh://git@servername/path/repo.git"
    );
}
