#[derive(Debug)]
struct Has<'lifetime> {
    lifetime: &'lifetime str,
}
fn main() {
    let long = String::from("long");
    let has02 = &long.clone();
    let mut has = Has { lifetime: &long };
    let mut has03 = Has { lifetime: has02 };
    dbg!(&has);
    {
        let short = String::from("short");
        // "switch" to short lifetime
        has.lifetime = &short;
        dbg!(has);

        let short02 = String::from("short02");
        has03.lifetime = &short02;
        dbg!(has03);
    }
}
