fn get_obj() -> Option<bool> {
    Some(true)
}

fn main() {
    let obj = get_obj();

    if let Some(o) = obj {
        // breakpoint on vvvvv
        println!("{o}");
        // when you hit it you do not see 'o' in 'locals',
        // and it cannot be added to 'watch'
    }
}
