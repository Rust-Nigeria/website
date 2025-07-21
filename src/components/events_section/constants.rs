// use self::types::RustEvent

use crate::components::events_section::types::{RustEvent, Speaker};

const AKIN_AGUDA: Speaker = Speaker {
    name: "Akin Aguda",
    image: "/assets/images/speakers/akin.png",
    github: None,
    linkedin: None,
    x: None,
    description: None,
};

const BEKKA: Speaker = Speaker {
    name: "Bekka",
    image: "/assets/images/speakers/bekka.png",
    github: None,
    linkedin: None,
    x: None,
    description: None,
};

const CHINEDU: Speaker = Speaker {
    name: "Chinedu",
    image: "/assets/images/speakers/chinedu.png",
    github: None,
    linkedin: None,
    x: None,
    description: None,
};

pub const EVENTS: [RustEvent; 3] = [
    RustEvent {
        banner: "/assets/images/event-banners/rockstar.jpg",
        name: "Rust in Game Development",
        description: "Join us as we unravel and delve into the use of Rust in the creation of our most popular franchises",
        event_link: "/",
        date: "2025-07-10T17:23:07.675Z",
        speakers: &[
            &AKIN_AGUDA,
            &BEKKA,
            &CHINEDU
        ]
   },
    RustEvent {
        banner: "/assets/images/event-banners/coinbase.png",
        name: "Rust in Game Development",
        description: "Join us as we unravel and delve into the use of Rust in the creation of our most popular franchises",
        event_link: "/",
        date: "2025-07-10T17:23:07.675Z",
        speakers: &[
            &AKIN_AGUDA,
            &BEKKA,
            &CHINEDU
        ]
   },
    RustEvent {
        banner: "/assets/images/event-banners/palantir.jpg",
        name: "Rust in Game Development",
        description: "Join us as we unravel and delve into the use of Rust in the creation of our most popular franchises",
        event_link: "/",
        date: "2025-07-10T17:23:07.675Z",
        speakers: &[
            &AKIN_AGUDA,
            &BEKKA,
            &CHINEDU
        ]
   }
];
