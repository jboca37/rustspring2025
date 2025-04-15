fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("{}", tracker);
    };

    update();
    update();
}

fn main() {
    track_changes();
    track_changes();
}