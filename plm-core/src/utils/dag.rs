// Copyright 2023 Sylk Technologies
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

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Package<T> {
    pub name: T,
    pub dependencies: HashSet<T>,
}

#[derive(Clone, Debug)]
pub struct Dag<T: Eq + Hash + Clone> {
    pub nodes: HashMap<T, Package<T>>,
}

impl<T: Eq + Hash + Clone> Default for Dag<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash + Clone> Dag<T> {
    pub fn new() -> Self {
        Dag {
            nodes: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, package: Package<T>) -> Result<(), &'static str> {
        if self.nodes.contains_key(&package.name) {
            return Err("Package already exists.");
        }

        // Check for cyclic dependencies
        for dep in &package.dependencies {
            if self.has_cyclic_dependency(&package.name, dep) {
                return Err("Cyclic dependency detected.");
            }
        }

        self.nodes.insert(package.name.clone(), package);
        Ok(())
    }

    fn has_cyclic_dependency(&self, start: &T, current: &T) -> bool {
        if start == current {
            return true;
        }

        if let Some(package) = self.nodes.get(current) {
            for dep in &package.dependencies {
                if self.has_cyclic_dependency(start, dep) {
                    return true;
                }
            }
        }

        false
    }
}
