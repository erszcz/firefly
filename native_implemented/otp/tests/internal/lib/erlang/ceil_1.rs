test_stdout!(without_number_errors_badarg, "{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n{caught, error, badarg}\n");
test_stdout!(with_integer_returns_integer, "-1\n0\n1\n");
test_stdout!(with_float_round_up_to_next_integer, "-1\n-1\n0\n1\n1\n");
