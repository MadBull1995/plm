// Copyright 2023 PLM Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use semver::{Version, VersionReq};

/// Resolves the best matching version for each package, given a list of available versions and
/// a list of package requirements.
///
/// # Arguments
/// * `available_versions`: A HashMap where keys are package names and values are vectors of available versions.
/// * `package_requirements`: A HashMap where keys are package names and values are version requirements.
///
/// # Returns
/// A Result containing a HashMap where keys are package names and values are the resolved versions.
pub fn resolve_versions(
    available_versions: HashMap<String, Vec<String>>,
    package_requirements: HashMap<String, String>,
) -> Result<HashMap<String, String>, String> {
    let mut resolved = HashMap::new();

    for (package, req_str) in package_requirements.iter() {
        let available = match available_versions.get(package) {
            Some(v) => v,
            None => return Err(format!("Package {} not found", package)),
        };

        let req = match VersionReq::parse(req_str) {
            Ok(r) => r,
            Err(_) => return Err(format!("Invalid version requirement for {}", package)),
        };

        let mut max_version: Option<Version> = None;
        for version_str in available {
            let version = match Version::parse(version_str) {
                Ok(v) => v,
                Err(_) => return Err(format!("Invalid version {} for {}", version_str, package)),
            };

            if req.matches(&version) {
                max_version = Some(match max_version {
                    Some(ref max) if &version > max => version.clone(),
                    Some(ref max) => max.clone(),
                    None => version.clone(),
                });
            }
        }

        match max_version {
            Some(v) => {
                resolved.insert(package.clone(), v.to_string());
            }
            None => return Err(format!("No matching version found for {}", package)),
        }
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_versions() {
        let mut available_versions = HashMap::new();
        available_versions.insert(
            "package_a".to_string(),
            vec!["1.0.0".to_string(), "2.0.0".to_string()],
        );
        available_versions.insert(
            "package_b".to_string(),
            vec!["0.9.0".to_string(), "1.0.0".to_string()],
        );

        let mut package_requirements = HashMap::new();
        package_requirements.insert("package_a".to_string(), "^1".to_string());
        package_requirements.insert("package_b".to_string(), ">=0.9, <2".to_string());

        let resolved = resolve_versions(available_versions, package_requirements).unwrap();

        assert_eq!(resolved.get("package_a").unwrap(), "1.0.0");
        assert_eq!(resolved.get("package_b").unwrap(), "1.0.0");
    }
}
