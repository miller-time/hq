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
        let next = fields.get(0).cloned();
        HclDeleter {
            fields,
            current_index: 0,
            current: next,
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
        let should_remove = self.should_remove();
        if let Some(ref curr) = self.current.clone() {
            for (index, item) in node.clone().iter().enumerate() {
                match item {
                    Structure::Attribute(attr) => {
                        if attr.key.as_str() == curr.name && should_remove {
                            node.remove_attribute(attr.key.as_str());
                        }
                    }
                    Structure::Block(block) => {
                        if block.ident.as_str() == curr.name && should_remove {
                            if curr.labels.is_empty() {
                                node.remove(index);
                            } else {
                                for filter_label in &curr.labels {
                                    for block_label in &block.labels {
                                        if block_label.as_str() == filter_label {
                                            node.remove_blocks(block.ident.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // check again for matches, these indicate that there are additional filter segments
            // (because if there was a match above, then the matching item is already gone)
            for attr in node.attributes_mut() {
                self.next_field();
                if attr.key.as_str() == curr.name {
                    self.visit_attr_mut(attr);
                }
                self.previous_field();
            }

            for block in node.blocks_mut() {
                self.next_field();
                if block.ident.as_str() == curr.name {
                    if curr.labels.is_empty() {
                        // then visit the body
                        self.visit_body_mut(&mut block.body);
                    } else {
                        for filter_label in &curr.labels {
                            for block_label in &block.labels {
                                if block_label.as_str() == filter_label {
                                    // then visit the body
                                    self.visit_body_mut(&mut block.body);
                                }
                            }
                        }
                    }
                }
                self.previous_field();
            }
        }
    }

    fn visit_object_mut(&mut self, node: &mut hcl_edit::expr::Object) {
        if let Some(ref curr) = self.current.clone() {
            let mut matches = Vec::new();
            for (key, _) in node.iter() {
                // some objects are keyed with an Ident
                if let Some(id) = key.as_ident() {
                    if id.as_str() == curr.name {
                        matches.push(key.clone());
                    }
                }
                // some objects are keyed with a String Expression
                if let Some(expr) = key.as_expr() {
                    if let Some(expr) = expr.as_str() {
                        if expr == curr.name {
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
