fn get_obj() -> Option<bool> {
    Some(true)
}

fn main() {
    if let Some(o) = get_obj() {
        // breakpoint on vvvvv
        println!("{o}");
        // when you hit it you do not see 'o' in 'locals',
        // and it cannot be added to 'watch'
    }
}
