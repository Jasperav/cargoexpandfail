macro_rules! do_nothing(
    () => {}
);

fn do_nothing_fn() {
    do_nothing!();
}