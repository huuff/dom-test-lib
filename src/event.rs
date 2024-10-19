pub fn new_change_event() -> web_sys::Event {
    let event_init = web_sys::EventInit::new();
    event_init.set_bubbles(true);
    web_sys::Event::new_with_event_init_dict("change", &event_init).unwrap()
}
