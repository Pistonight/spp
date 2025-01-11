fn main() {
    my_func();
    my_func2();
}

fn my_func() {
    let total_steps = 500;
    let progress = spp::printer(total_steps, "Doing something");
    // You can also:
    // use spp::Printer;
    // let progress = Printer::new(...);

    for i in 0..total_steps {
        std::thread::sleep(std::time::Duration::from_millis(10));
        // here you can print for example what's the current file
        // being worked on, etc...
        progress.print(i, format!("current step: {i}"))

        // do your work...
    }
}

fn my_func2() {
    let progress = spp::printer(10, "One");
    for i in 0..10 {
        progress.update(i)
    }
    // re-assign will drop the first one
    let progress = spp::printer(20, "Two");
    for i in 0..20 {
        progress.update(i)
    }
    // manually dropping to finish
    drop(progress);

    let progress2 = spp::printer(20, "Three");
    for i in 0..20 {
        progress2.update(i)
    }
    // automatically dropped
}
