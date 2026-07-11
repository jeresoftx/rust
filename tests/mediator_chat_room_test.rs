use design_patterns_rust::patterns::gof::behavioral::mediator::chat_room::ChatRoom;

#[test]
fn mediator_broadcasts_messages_without_users_knowing_each_other() {
    let mut room = ChatRoom::new();
    room.join("ana");
    room.join("luis");
    room.join("marta");

    room.broadcast("ana", "hola equipo");

    assert_eq!(room.inbox("ana"), Vec::<String>::new());
    assert_eq!(room.inbox("luis"), vec!["ana: hola equipo".to_string()]);
    assert_eq!(room.inbox("marta"), vec!["ana: hola equipo".to_string()]);
}

#[test]
fn mediator_routes_private_messages_to_one_user() {
    let mut room = ChatRoom::new();
    room.join("ana");
    room.join("luis");
    room.join("marta");

    let delivered = room.send_private("luis", "marta", "revisa el ticket");

    assert!(delivered);
    assert_eq!(room.inbox("ana"), Vec::<String>::new());
    assert_eq!(
        room.inbox("marta"),
        vec!["luis -> you: revisa el ticket".to_string()]
    );
}

#[test]
fn mediator_reports_private_message_delivery_failure() {
    let mut room = ChatRoom::new();
    room.join("ana");

    let delivered = room.send_private("ana", "sofia", "ping");

    assert!(!delivered);
    assert_eq!(
        room.inbox("ana"),
        vec!["system: sofia is not in the room".to_string()]
    );
}
