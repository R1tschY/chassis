#[test]
fn test_lib_deps() {
    version_sync::assert_contains_regex!("src/lib.rs", "chassis = \"\\^{version}\"");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
