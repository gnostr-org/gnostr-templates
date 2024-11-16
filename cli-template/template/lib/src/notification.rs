use mac_notification_sys::{get_bundle_identifier_or_default, send_notification, set_application};

pub(crate) fn notification() {
    let bundle = get_bundle_identifier_or_default("org.gnostr.org");
    println!("Bundle: {:?}", bundle);
    set_application(&bundle).unwrap();
    send_notification(
        "Danger",
        &Some("Will Robinson"),
        "Run away as fast as you can",
        &Some("Blow"),
    )
    .unwrap();
    //    send_notification("NOW", &None, "Without subtitle", &Some("Submarine")).unwrap();
}
