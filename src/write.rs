//! use the [`hcl-edit`][hcl_edit] crate to modify HCL documents

use hcl_edit::{
    expr::Expression,
    structure::{Attribute, Body, Structure},
    visit_mut::VisitMut,
    Decorate, Decorated, Ident,
};

use crate::parser::Field;

struct HclEditor<'a> {
    fields: Vec<Field>,
    current_index: usize,
    current: Option<Field>,
    value: &'a Expression,
}

impl<'a> HclEditor<'a> {
    fn new(fields: Vec<Field>, value: &'a Expression) -> Self {
        let current = fields.first().cloned();
        HclEditor {
            fields,
            current_index: 0,
            current,
            value,
        }
    }

    fn current_field(&self) -> Option<Field> {
        self.fields.get(self.current_index).cloned()
    }

    fn next_field(&mut self) {
        self.current_index += 1;
        self.current = self.current_field();
    }

    fn previous_field(&mut self) {
        self.current_index -= 1;
        self.current = self.current_field();
    }

    fn should_edit(&self) -> bool {
        self.current_index >= self.fields.len() - 1
    }
}

impl VisitMut for HclEditor<'_> {
    fn visit_body_mut(&mut self, node: &mut Body) {
        if let Some(current) = self.current.clone() {
            let mut matching_attr_keys = Vec::new();
            let mut matching_block_idents = Vec::new();
            // save this in case we are adding new attributes
            let mut decor = None;
            for item in node.iter() {
                match item {
                    Structure::Attribute(attr) => {
                        // copy existing attribute's decor
                        decor = Some(attr.decor().clone());
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
                self.next_field();
                self.visit_attr_mut(node.get_attribute_mut(&key).unwrap());
                self.previous_field();
            }

            for ident in matching_block_idents {
                for block in node.get_blocks_mut(&ident) {
                    self.next_field();
                    self.visit_body_mut(&mut block.body);
                    self.previous_field();
                }
            }

            if self.should_edit() {
                let key = Decorated::new(Ident::new(current.name));
                // copy existing attribute's decor when creating the new attribute
                let decor = decor.unwrap_or_default();
                let attr = Attribute::new(key, self.value.clone()).decorated(decor);
                node.insert(node.len(), attr);
            }
        }
    }

    fn visit_attr_mut(&mut self, mut node: hcl_edit::structure::AttributeMut) {
        if self.should_edit() {
            let value = node.value_mut();
            *value = self.value.clone();
        } else {
            self.next_field();
            self.visit_expr_mut(node.value_mut());
            self.previous_field();
        }
    }
}

/// given a vector of [`Field`]s, write `value` to replace the existing
/// [`Expression`] that matches that filter
pub fn write(fields: Vec<Field>, body: &mut Body, value: &Expression) {
    let mut visitor = HclEditor::new(fields, value);
    visitor.visit_body_mut(body);
}
