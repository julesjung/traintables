use serde::Serialize;

#[derive(Serialize)]
struct Station {
    id: String,
    name: String,
    code: String,
    longitude: String,
    latitude: String,
}
