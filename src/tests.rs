use crate::utils::get_id_from_link;
use crate::Kutt;

#[test]
fn short_url_test() {
    let slink = Kutt::target_url("https://google.com/great")
        .reuse()
        .create_short_link()
        .unwrap();
    assert_eq!(slink.starts_with("https://kutt.it/"), true);
    assert_eq!(Kutt::delete_link(slink.as_str()).is_ok(), true);
}

#[test]
fn custom_url_and_delete_test() {
    let slink = Kutt::target_url("https://google.com/foobar")
        .custom_url("foobarfoobarfoobarfoobar")
        .reuse()
        .create_short_link()
        .unwrap();
    assert_eq!(slink.as_str(), "https://kutt.it/foobarfoobarfoobarfoobar");
    assert_eq!(Kutt::delete_link(slink.as_str()).is_ok(), true);
}

#[test]
fn get_id_from_link_test() {
    let id = get_id_from_link("https://kutt.it/foobarfoo").unwrap();
    assert_eq!(id, "foobarfoo");
}
