#[derive(Debug)]
pub enum FragmentSegment {
    Single(String),
    // Segments(Vec<FragmentArgument>),
}

pub fn parse_route_fragment(route: &str) -> Option<FragmentSegment> {
    route
        .rsplit_once('#')
        .and_then(|(_, fragment)| Some(FragmentSegment::Single(fragment.to_string())))
}
