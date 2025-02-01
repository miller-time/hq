some_attr = {
    foo = [1, 2]
    bar = true
}

some_block "some_block_label" {
    attr = "value"
}

some_block "another_block_label" {
    attr = "another_value"
}

# this is a block comment
fmt_block "fmt_label" {
    # this is a body comment
    # this is another body comment

    # this is a third body comment
    first_formatted_field  = "fmt_value"
    second_formatted_field = "second_value"
}

nested_block {
    inner_block {
        value = "deep"
        another_value = "nested"
    }
}
