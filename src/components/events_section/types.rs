pub struct Speaker {
    pub image: &'static str,
    pub name: &'static str,
    pub github: Option<&'static str>,
    pub linkedin: Option<&'static str>,
    pub x: Option<&'static str>,
    pub description: Option<&'static str>,
}

pub struct RustEvent {
    pub banner: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub speakers: &'static [&'static Speaker],
    pub event_link: &'static str,
    pub date: &'static str,
}
