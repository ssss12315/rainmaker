#[test]
fn parse_minimal_config_like_readme() {
    // This is a smoke test to ensure the config structure shown in README stays parseable.
    // If your project uses serde to parse config, you can plug the actual types here.
    let yml = r#"
core:
  rpc_urls:
    - "https://testnet-rpc.monad.xyz"
  target_tps: 200
  rpc_batch_size: 100
  distribution_type: "native-direct"
  addresses_file: "addresses.txt"
"#;

    // Example: replace with your real config struct if available.
    // For a placeholder, we just ensure YAML is syntactically valid.
    let parsed: serde_yaml::Value = serde_yaml::from_str(yml).expect("valid YAML");
    assert!(parsed.get("core").is_some());
}
