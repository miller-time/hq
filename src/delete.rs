//! use the [`hcl-edit`][hcl_edit] crate to remove values from HCL documents

use std::error::Error;

use hcl_edit::{
    structure::{Body, Structure},
    visit_mut::VisitMut,
};

use crate::parser::Field;

struct HclDeleter {
    fields: Vec<Field>,
    next: Option<Field>,
    error: Option<Box<dyn Error>>,
}

impl HclDeleter {
    fn new(fields: Vec<Field>) -> Self {
        HclDeleter {
            fields,
            next: None,
            error: None,
        }
    }

    fn next_field(&mut self) {
        if !self.fields.is_empty() {
            self.next = Some(self.fields.remove(0));
        }
    }

    fn should_remove(&self) -> bool {
        self.fields.is_empty()
    }
}

impl VisitMut for HclDeleter {
    fn visit_body_mut(&mut self, node: &mut Body) {
        self.next_field();
        // create a clone so that we can later mutate `self.next`
        let next = self.next.clone();
        if let Some(ref next) = next {
            for (index, item) in node.clone().iter().enumerate() {
                match item {
                    Structure::Attribute(attr) => {
                        if attr.key.as_str() == next.name && self.should_remove() {
                            node.remove_attribute(attr.key.as_str());
                        }
                    }
                    Structure::Block(block) => {
                        if block.ident.as_str() == next.name && self.should_remove() {
                            if next.labels.is_empty() {
                                node.remove(index);
                            } else {
                                for filter_label in &next.labels {
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
                if attr.key.as_str() == next.name {
                    self.visit_attr_mut(attr);
                }
            }

            for block in node.blocks_mut() {
                if block.ident.as_str() == next.name {
                    if next.labels.is_empty() {
                        // then visit the body
                        self.visit_body_mut(&mut block.body);
                    } else {
                        for filter_label in &next.labels {
                            for block_label in &block.labels {
                                if block_label.as_str() == filter_label {
                                    // then visit the body
                                    self.visit_body_mut(&mut block.body);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn visit_object_mut(&mut self, node: &mut hcl_edit::expr::Object) {
        self.next_field();
        if let Some(ref next) = self.next {
            let mut matches = Vec::new();
            for (key, _) in node.iter() {
                // some objects are keyed with an Ident
                if let Some(id) = key.as_ident() {
                    if id.as_str() == next.name {
                        matches.push(key.clone());
                    }
                }
                // some objects are keyed with a String Expression
                if let Some(expr) = key.as_expr() {
                    if let Some(expr) = expr.as_str() {
                        if expr == next.name {
                            matches.push(key.clone());
                        }
                    }
                }
            }
            for key in matches {
                node.remove(&key);
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
