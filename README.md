# spp
Simple progress printer for rust

```
[123/456] Doing something: 26.97% ETA 35.20s current step ...
```

The progress style is inspired by ninja (the build tool)

## Features
- Prints to stderr to integrate with pipes easily (final status printed to stdout)
- Handles throttling internally
- Display ETA after 2 seconds
- Display current job
- Display current step in the job (for example which file is currently
  being processed, etc)

## Install
There's already a million progress printing library on crates.io,
so I don't intend to publish mine there. However if you want to use
mine you can directly install from GitHub
```bash
cargo add spp --git https://github.com/Pistonight/spp --branch main
```

## Usage
```rust

fn my_func() {
    let total_steps = 500;
    let progress = spp::printer(total_steps, "Doing something")
    // You can also:
    // use spp::Printer;
    // let progress = Printer::new(...);

    for i in 0..total_steps {
        // here you can print for example what's the current file
        // being worked on, etc...
        progress.print(i, format!("current step: {i}"))

        // do your work...
    }
}

```

## Throttling
You can customize the throttling interval, which is 50ms by default
```rust
let mut progress = spp::printer(500, "Doing Something");
progress.set_throttle_duration(std::time::Duration::from_millis(200));
```

## Finishing
The progress is considered done when the printer is dropped.
Upon dropping, it will print the final message showing current = total steps,
and prints a newline.

You can manually drop it or assign it if there are multiple progresses
needed in the same scope
```rust
fn my_func() {
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
```
This will leave these in the console after finishing:
```
[10/10] One
[20/20] Two
[20/20] Three
```
