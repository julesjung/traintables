use csv_async::AsyncSerializer;
use futures_util::TryStreamExt;
use quick_xml::{events::Event, reader::Reader};
use serde::Serialize;
use std::io;
use tokio::fs::File;
use tokio_util::io::StreamReader;

#[derive(Serialize, Default)]
struct TripUpdate {
    date: String,
    trip_id: String,
    cancelled: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let url = "https://proxy.transport.data.gouv.fr/resource/sncf-siri-lite-estimated-timetable";

    let response = reqwest::get(url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let stream = response
        .bytes_stream()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e));
    let stream_reader = StreamReader::new(stream);
    let mut xml_reader = Reader::from_reader(stream_reader);

    let file = File::create("build/trip_updates.csv").await?;
    let mut writer = AsyncSerializer::from_writer(file);

    let mut buf = Vec::new();
    let mut current_update: Option<TripUpdate> = None;
    let mut current_tag: Option<Vec<u8>> = None;

    loop {
        let event = xml_reader
            .read_event_into_async(&mut buf)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        match event {
            Event::Eof => break,
            Event::Start(e) => match e.name().as_ref() {
                b"EstimatedVehicleJourney" => current_update = Some(TripUpdate::default()),
                tag => current_tag = Some(tag.to_vec()),
            },
            Event::Text(e) => {
                if let (Some(update), Some(tag)) = (current_update.as_mut(), current_tag.as_deref())
                {
                    match tag {
                        b"DatedVehicleJourneyRef" => {
                            update.trip_id = e.decode().unwrap().into_owned()
                        }
                        _ => (),
                    }
                }
            }
            Event::End(e) => match e.name().as_ref() {
                b"EstimatedVehicleJourney" => writer.serialize(&current_update).await?,
                _ => (),
            },
            _ => (),
        };

        buf.clear();
    }

    Ok(())
}
