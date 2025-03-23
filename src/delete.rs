//! use the [`hcl-edit`][hcl_edit] crate to remove values from HCL documents

use std::error::Error;

use hcl_edit::{
    structure::{Body, Structure},
    visit_mut::VisitMut,
};

use crate::parser::Field;

struct HclDeleter {
    fields: Vec<Field>,
    current_index: usize,
    current: Option<Field>,
    error: Option<Box<dyn Error>>,
}

impl HclDeleter {
    fn new(fields: Vec<Field>) -> Self {
        let current = fields.first().cloned();
        HclDeleter {
            fields,
            current_index: 0,
            current,
            error: None,
        }
    }

    fn next_field(&mut self) {
        self.current_index += 1;
        self.current = self.fields.get(self.current_index).cloned();
    }

    fn previous_field(&mut self) {
        self.current_index -= 1;
        self.current = self.fields.get(self.current_index).cloned();
    }

    fn should_remove(&self) -> bool {
        self.current_index >= self.fields.len() - 1
    }
}

impl VisitMut for HclDeleter {
    fn visit_body_mut(&mut self, node: &mut Body) {
        if let Some(current) = self.current.clone() {
            let mut matching_attr_keys = Vec::new();
            let mut matching_block_idents = Vec::new();
            for item in node.iter() {
                match item {
                    Structure::Attribute(attr) => {
                        if attr.key.as_str() == current.name {
                            matching_attr_keys.push(attr.key.to_string());
                        }
                    }
                    Structure::Block(block) => {
                        if block.ident.as_str() == current.name {
                            if current.labels.is_empty() {
                                matching_block_idents.push(block.ident.to_string());
                            } else {
                                for filter_label in &current.labels {
                                    for block_label in &block.labels {
                                        if block_label.as_str() == filter_label {
                                            matching_block_idents.push(block.ident.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for key in matching_attr_keys {
                if self.should_remove() {
                    node.remove_attribute(&key);
                } else {
                    self.next_field();
                    // Key was gotten iterating over the node, so it must be a non-None value.
                    self.visit_attr_mut(node.get_attribute_mut(&key).unwrap());
                    self.previous_field();
                }
            }

            for ident in matching_block_idents {
                if self.should_remove() {
                    node.remove_blocks(&ident);
                } else {
                    for block in node.get_blocks_mut(&ident) {
                        self.next_field();
                        self.visit_block_mut(block);
                        self.previous_field();
                    }
                }
            }
        }
    }

    fn visit_object_mut(&mut self, node: &mut hcl_edit::expr::Object) {
        if let Some(current) = self.current.clone() {
            let mut matches = Vec::new();
            for (key, _) in node.iter() {
                // some objects are keyed with an Ident
                if let Some(id) = key.as_ident() {
                    if id.as_str() == current.name {
                        matches.push(key.clone());
                    }
                }
                // some objects are keyed with a String Expression
                if let Some(expr) = key.as_expr() {
                    if let Some(expr) = expr.as_str() {
                        if expr == current.name {
                            matches.push(key.clone());
                        }
                    }
                }
            }

            for key in matches {
                if self.should_remove() {
                    node.remove(&key);
                } else if let Some(val) = node.get_mut(&key) {
                    // If we haven't reached the end of the query, we need to traverse further into
                    // the AST to determine what needs to be deleted.
                    self.next_field();
                    self.visit_object_value_mut(val);
                    self.previous_field();
                } else {
                    // Every key in this vec was gotten by iterating over this object, so the value
                    // should exist and this branch should not be reachable.
                    unreachable!();
                }
            }
        }
    }
}

/// given a vector of [`Field`]s, delete the [`Expression`] value that matches that filter
pub fn delete(fields: Vec<Field>, body: &mut Body) -> Result<(), Box<dyn Error>> {
    let mut visitor = HclDeleter::new(fields);
    visitor.visit_body_mut(body);
    if let Some(err) = visitor.error {
        return Err(err);
    }
    Ok(())
}
