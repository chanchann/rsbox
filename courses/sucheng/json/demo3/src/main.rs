fn main() {
    let user_str = r#"
    {
        "id" : 10,
        "user_name" : "guest",
        "friends" : ["chan", "yi"],
        "roles" : [
            {"name" : "admin"},
            {"name" : "guest"}
        ]
    }
    "#;

    let user:serde_json::Value = serde_json::from_str(user_str).unwrap();
    println!("{}", user.get("id").unwrap());

    println!("{}", user.get("roles").unwrap().get(0).unwrap().get("name").unwrap());

    println!("{}", user.as_object().and_then(|v| v.get("user_name")).unwrap().as_str().unwrap());

    println!("{}", user.as_object().and_then(|v| v.get("friends"))
                                   .and_then(|v| v.get(0)).unwrap());
}
