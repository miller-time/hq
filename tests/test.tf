variable "my_var" { default = "my_default_value" }
variable "another_var" { default = "another_default_value" }

data "a_data_block" "with_some_attrs" {
    my_attr = "my_attr_value"
    another_attr = "another_attr_value"
}

data "another_data_block" "with_some_attrs" {
    cromulent_attr = "cromulent_value"
}
