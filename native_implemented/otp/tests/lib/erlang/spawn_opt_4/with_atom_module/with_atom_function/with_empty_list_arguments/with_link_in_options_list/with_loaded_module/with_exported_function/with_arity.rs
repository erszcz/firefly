test_stdout!(
    with_exits_normal_arent_does_not_exit,
    "{in, child}\n{parent, alive, true}\n"
);
test_stdout_substrings!(
    without_exits_normal_arent_does_not_exit,
    vec![
        "{in, child}",
        "exited with reason: abnormal",
        "{parent, exited, abnormal}"
    ]
);
