//! use the [`hcl-edit`][hcl_edit] crate to modify HCL documents

use std::{error::Error, fmt};

use hcl_edit::{expr::Expression, structure::Body, visit_mut::VisitMut};

use crate::parser::Field;

#[derive(Debug)]
pub struct WriteError {
    reason: String,
}

impl WriteError {
    fn new(reason: &str) -> Self {
        WriteError {
            reason: reason.to_string(),
        }
    }
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to write HCL: {}", self.reason)
    }
}

impl Error for WriteError {}

fn err(reason: &str) -> Box<WriteError> {
    Box::new(WriteError::new(reason))
}

struct HclEditor<'a> {
    fields: Vec<Field>,
    next: Option<Field>,
    value: &'a Expression,
    error: Option<Box<WriteError>>,
}

impl<'a> HclEditor<'a> {
    fn new(fields: Vec<Field>, value: &'a Expression) -> Self {
        HclEditor {
            fields,
            next: None,
            value,
            error: None,
        }
    }

    fn next_field(&mut self) {
        if self.next.is_none() && !self.fields.is_empty() {
            self.next = Some(self.fields.remove(0));
        }
    }
}

impl VisitMut for HclEditor<'_> {
    fn visit_attr_mut(&mut self, mut node: hcl_edit::structure::AttributeMut) {
        self.next_field();
        // perform update if the attr key matches the field
        if let Some(ref next) = self.next {
            if node.key.as_str() == next.name {
                let value = node.value_mut();
                *value = self.value.clone();
            }
        }
    }

    fn visit_block_mut(&mut self, node: &mut hcl_edit::structure::Block) {
        self.next_field();
        // create a clone so that we can later mutate `self.next`
        let next = self.next.clone();
        if let Some(next) = next {
            if node.ident.as_str() == next.name {
                if next.labels.is_empty() {
                    if self.fields.is_empty() {
                        self.error = Some(err("unable to write expr as block body"));
                        return;
                    }
                    // the block is a match if its name matches and there are no labels
                    // traverse to the next field
                    self.next = Some(self.fields.remove(0));
                    // then visit the body
                    self.visit_body_mut(&mut node.body);
                } else {
                    for filter_label in &next.labels {
                        for block_label in &node.labels {
                            if block_label.as_str() == filter_label {
                                if self.fields.is_empty() {
                                    self.error = Some(err("unable to write expr as block body"));
                                    return;
                                }
                                // the block name and this label match the filters
                                // traverse to the next field
                                self.next = Some(self.fields.remove(0));
                                // then visit the body
                                self.visit_body_mut(&mut node.body);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// given a vector of [`Field`]s, write `value` to replace the existing
/// [`Expression`] that matches that filter
pub fn write(
    fields: Vec<Field>,
    body: &mut Body,
    value: &Expression,
) -> Result<(), Box<WriteError>> {
    let mut visitor = HclEditor::new(fields, value);
    visitor.visit_body_mut(body);
    if let Some(err) = visitor.error {
        return Err(err);
    }
    Ok(())
}
