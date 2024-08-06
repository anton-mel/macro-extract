mod watcher;

fn main() -> notify::Result<()> {
    watcher::start_watching()
}
