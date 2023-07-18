fn main() {
    use mylib::strategy::conceptural;

    let navigator = conceptural::Navigator::new(conceptural::WalkingStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = conceptural::Navigator::new(conceptural::PublicTransportStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    use mylib::strategy::functional;

    let navigator = functional::Navigator::new(functional::walking_strategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = functional::Navigator::new(functional::public_transport_strategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = functional::Navigator::new(|from: &str, to: &str| {
        println!("Clouser route {from} to{to}: 1km, 0.1s")
    });
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}
