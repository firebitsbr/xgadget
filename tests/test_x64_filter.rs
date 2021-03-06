mod common;

#[test]
fn test_x64_filter_stack_pivot() {
    let bin_filters = common::get_raw_bin("bin_filters", &common::FILTERS_X64);
    let bins = vec![bin_filters];
    let gadgets =
        xgadget::find_gadgets(&bins, common::MAX_LEN, xgadget::SearchConfig::DEFAULT).unwrap();
    let stack_pivot_gadgets = xgadget::filter_stack_pivot(&gadgets);
    let stack_pivot_gadget_strs = common::get_gadget_strs(&stack_pivot_gadgets, false);
    common::print_gadget_strs(&stack_pivot_gadget_strs);

    // Positive
    assert!(common::gadget_strs_contains_sub_str(
        &stack_pivot_gadget_strs,
        "pop rsp; ret;"
    ));

    // Negative
    assert!(!common::gadget_strs_contains_sub_str(
        &stack_pivot_gadget_strs,
        "pop rax; pop rbx; ret;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &stack_pivot_gadget_strs,
        "pop rbx; ret;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &stack_pivot_gadget_strs,
        "add rax, 0x08; jmp rax;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &stack_pivot_gadget_strs,
        "mov rax, 0x1337; jmp [rax];"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &stack_pivot_gadget_strs,
        "pop rax; jmp rax;"
    ));
}

#[test]
fn test_x64_filter_dispatcher() {
    let bin_filters = common::get_raw_bin("bin_filters", &common::FILTERS_X64);
    let bins = vec![bin_filters];
    let gadgets =
        xgadget::find_gadgets(&bins, common::MAX_LEN, xgadget::SearchConfig::DEFAULT).unwrap();
    let dispatcher_gadgets = xgadget::filter_dispatcher(&gadgets);
    let dispatcher_gadget_strs = common::get_gadget_strs(&dispatcher_gadgets, false);
    common::print_gadget_strs(&dispatcher_gadget_strs);

    // Positive
    assert!(common::gadget_strs_contains_sub_str(
        &dispatcher_gadget_strs,
        "add rax, 0x8; jmp rax;"
    ));

    // Negative
    assert!(!common::gadget_strs_contains_sub_str(
        &dispatcher_gadget_strs,
        "pop rsp; ret;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &dispatcher_gadget_strs,
        "pop rax; pop rbx; ret;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &dispatcher_gadget_strs,
        "pop rbx; ret;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &dispatcher_gadget_strs,
        "mov rax, 0x1337; jmp [rax];"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &dispatcher_gadget_strs,
        "pop rax; jmp rax;"
    ));
}

#[test]
fn test_x64_filter_stack_set_regs() {
    let bin_filters = common::get_raw_bin("bin_filters", &common::FILTERS_X64);
    let bins = vec![bin_filters];
    let gadgets =
        xgadget::find_gadgets(&bins, common::MAX_LEN, xgadget::SearchConfig::DEFAULT).unwrap();
    let loader_gadgets = xgadget::filter_stack_set_regs(&gadgets);
    let loader_gadget_strs = common::get_gadget_strs(&loader_gadgets, false);
    common::print_gadget_strs(&loader_gadget_strs);

    // Positive
    assert!(common::gadget_strs_contains_sub_str(
        &loader_gadget_strs,
        "pop rsp; ret;"
    ));
    assert!(common::gadget_strs_contains_sub_str(
        &loader_gadget_strs,
        "pop rax; pop rbx; ret;"
    ));
    assert!(common::gadget_strs_contains_sub_str(
        &loader_gadget_strs,
        "pop rbx; ret;"
    ));
    assert!(common::gadget_strs_contains_sub_str(
        &loader_gadget_strs,
        "pop rax; jmp rax;"
    ));

    // Negative
    assert!(!common::gadget_strs_contains_sub_str(
        &loader_gadget_strs,
        "add rax, 0x08; jmp rax;"
    ));
    assert!(!common::gadget_strs_contains_sub_str(
        &loader_gadget_strs,
        "mov rax, 0x1337; jmp [rax];"
    ));
}

#[test]
fn test_x64_filter_bad_bytes() {
    let bin_filters = common::get_raw_bin("bin_filters", &common::FILTERS_X64);
    let bins = vec![bin_filters];
    let gadgets =
        xgadget::find_gadgets(&bins, common::MAX_LEN, xgadget::SearchConfig::DEFAULT).unwrap();
    let gadget_strs = common::get_gadget_strs(&gadgets, false);
    let good_bytes_gadgets =
        xgadget::filter_bad_addr_bytes(&gadgets, &[0x10, 0x14, 0x15, 0xc, 0xd]);
    let good_bytes_gadget_strs = common::get_gadget_strs(&good_bytes_gadgets, false);
    common::print_gadget_strs(&good_bytes_gadget_strs);

    // Positive
    assert!(!common::gadget_strs_contains_sub_str(
        &good_bytes_gadget_strs,
        "jmp rax;"
    ));

    // Negative
    assert!(common::gadget_strs_contains_sub_str(
        &gadget_strs,
        "jmp rax;"
    ));
}
