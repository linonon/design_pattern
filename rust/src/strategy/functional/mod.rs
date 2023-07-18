pub type RouteStrategy = fn(from: &str, to: &str);

pub fn walking_strategy(from: &str, to: &str) {
    println!("Walking route from {from} to {to}: 4km, 30min");
}

pub fn public_transport_strategy(from: &str, to: &str) {
    println!("Public transport from {from} to {to}: 3km, 5min");
}

pub struct Navigator {
    route_startegy: RouteStrategy,
}

impl Navigator {
    pub fn new(route_startegy: RouteStrategy) -> Self {
        Self { route_startegy }
    }

    pub fn route(&self, from: &str, to: &str) {
        (self.route_startegy)(from, to);
    }
}
