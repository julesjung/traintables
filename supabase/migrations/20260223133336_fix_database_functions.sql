drop function public.get_arrivals(text, date, integer);
create function public.get_arrivals(
    at_station_id text,
    on_date date,
    from_seconds integer
) returns table(
    trip_id text,
    route_short_name text,
    origin text,
    arrival_time timestamp without time zone,
    headsign text,
    color text,
    text_color text
) language plpgsql
set search_path to ' '
as $$
declare
    search_start timestamp;
    search_end timestamp;
begin
    search_start := on_date + from_seconds * interval '1 second';
    search_end := search_start + interval '1 day';

    return query
    select
        trips.id as trip_id,
        routes.short_name as route_short_name,
        origin_stations.name as origin,
        service_days.date + stop_times.arrival_seconds * interval '1 second' as arrival_time,
        trips.headsign,
        routes.color,
        routes.text_color
    from public.stop_times
    join public.stops on stops.id = stop_times.stop_id
    join public.trips on trips.id = stop_times.trip_id
    join public.routes on routes.id = trips.route_id
    join public.service_days on trips.service_id = service_days.service_id
    join public.stops as origin_stops on origin_stops.id = trips.origin_id
    join public.stations as origin_stations on origin_stations.id = origin_stops.station_id
    where stops.station_id = at_station_id
    and service_days.date + stop_times.arrival_seconds * interval '1 second' >= search_start
    and service_days.date + stop_times.arrival_seconds * interval '1 second' < search_end
    and at_station_id != origin_stations.id
    order by service_days.date + stop_times.arrival_seconds * interval '1 second'
    limit 50;
end;
$$;

drop function public.get_departures(text, date, integer);
create function public.get_departures(
    at_station_id text,
    on_date date,
    from_seconds integer
) returns table(
    trip_id text,
    route_short_name text,
    destination text,
    departure_time timestamp without time zone,
    headsign text,
    color text,
    text_color text
) language plpgsql
set search_path to ' '
as $$
declare
    search_start timestamp;
    search_end timestamp;
begin
    search_start := on_date + from_seconds * interval '1 second';
    search_end := search_start + interval '1 day';

    return query
    select
        trips.id as trip_id,
        routes.short_name as route_short_name,
        destination_stations.name as destination,
        service_days.date + stop_times.departure_seconds * interval '1 second' as departure_time,
        trips.headsign,
        routes.color,
        routes.text_color
    from public.stop_times
    join public.stops on stops.id = stop_times.stop_id
    join public.trips on trips.id = stop_times.trip_id
    join public.routes on routes.id = trips.route_id
    join public.service_days on trips.service_id = service_days.service_id
    join public.stops as destination_stops on destination_stops.id = trips.destination_id
    join public.stations as destination_stations on destination_stations.id = destination_stops.station_id
    where stops.station_id = at_station_id
    and service_days.date + stop_times.departure_seconds * interval '1 second' >= search_start
    and service_days.date + stop_times.departure_seconds * interval '1 second' < search_end
    and at_station_id != destination_stations.id
    order by service_days.date + stop_times.departure_seconds * interval '1 second'
    limit 50;
end;
$$;

drop function public.get_trip_stops(text, date);
create function public.get_trip_stops(
    on_trip_id text,
    on_date date
) returns table(
    name text,
    arrival_date timestamp without time zone,
    departure_date timestamp without time zone
) language plpgsql
set search_path to ' '
as $$
begin
    return query
    select
        stops.name,
        service_days.date + stop_times.arrival_seconds * interval '1 second' as arrival_date,
        service_days.date + stop_times.departure_seconds * interval '1 second' as departure_date
    from public.stop_times
    join public.stops on stops.id = stop_times.stop_id
    join public.trips on trips.id = stop_times.trip_id
    join public.service_days on service_days.service_id = trips.service_id
    where stop_times.trip_id = on_trip_id
    and service_days.date = on_date
    order by stop_times.stop_sequence;
end;
$$;
