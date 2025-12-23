CREATE TABLE stations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL
);

CREATE TABLE stop_points (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    station_id TEXT NOT NULL,
    FOREIGN KEY (station_id) REFERENCES stations(id)
);

CREATE TABLE routes (
    id TEXT PRIMARY KEY,
    short_name TEXT NOT NULL,
    long_name TEXT NOT NULL,
    type INT NOT NULL,
    color TEXT,
    text_color TEXT
);

CREATE TABLE trips (
    id TEXT PRIMARY KEY,
    route_id TEXT NOT NULL,
    headsign INT NOT NULL,
    direction INT,
    FOREIGN KEY (route_id) REFERENCES routes(id)
);

CREATE TABLE stop_times (
    trip_id TEXT NOT NULL,
    arrival_time TEXT NOT NULL,
    departure_time TEXT NOT NULL,
    stop_id TEXT NOT NULL,
    stop_sequence INT NOT NULL,
    FOREIGN KEY (trip_id) REFERENCES trips(id),
    FOREIGN KEY (stop_id) REFERENCES stop_points(id)
);