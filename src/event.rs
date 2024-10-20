fn bubbles_evt_init() -> web_sys::EventInit {
    let event_init = web_sys::EventInit::new();
    event_init.set_bubbles(true);
    event_init
}

pub fn change_evt() -> web_sys::Event {
    web_sys::Event::new_with_event_init_dict("change", &bubbles_evt_init()).unwrap()
}

pub fn input_evt() -> web_sys::InputEvent {
    let evt = web_sys::InputEvent::new("input").unwrap();
    evt.init_event_with_bubbles("input", true);
    evt
}
