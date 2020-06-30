pub enum Publishers {
    BBC,
    SKYNEWS,
}

#[derive(Clone, Copy)]
pub struct Publisher {
    pub name: &'static str,
    pub profile_link: &'static str,
}