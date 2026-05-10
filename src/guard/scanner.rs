use serde_json::Value;

pub fn scan_workflow(workflow_json: &str) -> Result<(), String> {
    let v: Value = serde_json::from_str(workflow_json).map_err(|e| e.to_string())?;

    if let Some(nodes) = v.as_object() {
        for (_id, node) in nodes {
            if let Some(class_type) = node.get("class_type").and_then(|c| c.as_str()) {
                let lower_class = class_type.to_lowercase();

                // Blacklist keywords (case-insensitive substring)
                let keywords = ["python", "execute", "system", "script", "os"];
                for kw in keywords {
                    if lower_class.contains(kw) {
                        return Err(class_type.to_string());
                    }
                }

                // Blacklist specific node names (exact match)
                let specific_nodes = ["CustomNodeLoader", "TerminalCommand", "WebFetchNode"];
                for specific in specific_nodes {
                    if class_type == specific {
                        return Err(class_type.to_string());
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_workflow() {
        let json = r#"{"1": {"class_type": "CheckpointLoaderSimple"}}"#;
        assert!(scan_workflow(json).is_ok());
    }

    #[test]
    fn test_malicious_keyword() {
        let json = r#"{"1": {"class_type": "PythonScript"}}"#;
        assert_eq!(scan_workflow(json).unwrap_err(), "PythonScript");
    }

    #[test]
    fn test_malicious_specific() {
        let json = r#"{"1": {"class_type": "TerminalCommand"}}"#;
        assert_eq!(scan_workflow(json).unwrap_err(), "TerminalCommand");
    }
}
