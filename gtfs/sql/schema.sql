CREATE TABLE stations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL
);

CREATE TABLE stops (
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
    service_id INT NOT NULL,
    headsign TEXT NOT NULL,
    direction INT,
    origin TEXT NOT NULL,
    destination TEXT NOT NULL,
    FOREIGN KEY (route_id) REFERENCES routes(id),
    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (origin) REFERENCES stops(id),
    FOREIGN KEY (destination) REFERENCES stops(id)
);

CREATE TABLE stop_times (
    trip_id TEXT NOT NULL,
    arrival_time TEXT NOT NULL,
    departure_time TEXT NOT NULL,
    stop_id TEXT NOT NULL,
    stop_sequence INT NOT NULL,
    FOREIGN KEY (trip_id) REFERENCES trips(id),
    FOREIGN KEY (stop_id) REFERENCES stops(id)
);

CREATE TABLE services (
    id INT PRIMARY KEY
);

CREATE TABLE service_days (
    service_id INT NOT NULL,
    date TEXT NOT NULL,
    PRIMARY KEY (service_id, date),
    FOREIGN KEY (service_id) REFERENCES services(id)
);