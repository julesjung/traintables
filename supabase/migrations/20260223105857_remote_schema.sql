


SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;


COMMENT ON SCHEMA "public" IS 'standard public schema';



CREATE EXTENSION IF NOT EXISTS "pg_stat_statements" WITH SCHEMA "extensions";






CREATE EXTENSION IF NOT EXISTS "pgcrypto" WITH SCHEMA "extensions";






CREATE EXTENSION IF NOT EXISTS "supabase_vault" WITH SCHEMA "vault";






CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA "extensions";






CREATE OR REPLACE FUNCTION "public"."get_arrivals"("at_station_id" "text", "on_date" "date", "from_seconds" integer) RETURNS TABLE("trip_id" "text", "route_short_name" "text", "origin" "text", "arrival_time" timestamp without time zone, "arrival_delay" integer, "headsign" "text", "color" "text", "text_color" "text")
    LANGUAGE "plpgsql"
    SET "search_path" TO ''
    AS $$declare
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
end;$$;


ALTER FUNCTION "public"."get_arrivals"("at_station_id" "text", "on_date" "date", "from_seconds" integer) OWNER TO "postgres";


CREATE OR REPLACE FUNCTION "public"."get_departures"("at_station_id" "text", "on_date" "date", "from_seconds" integer) RETURNS TABLE("trip_id" "text", "route_short_name" "text", "destination" "text", "departure_time" timestamp without time zone, "departure_delay" integer, "headsign" "text", "color" "text", "text_color" "text")
    LANGUAGE "plpgsql"
    SET "search_path" TO ''
    AS $$declare
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
end;$$;


ALTER FUNCTION "public"."get_departures"("at_station_id" "text", "on_date" "date", "from_seconds" integer) OWNER TO "postgres";


CREATE OR REPLACE FUNCTION "public"."get_trip_stops"("on_trip_id" "text", "on_date" "date") RETURNS TABLE("name" "text", "arrival_date" timestamp without time zone, "arrival_delay" integer, "departure_date" timestamp without time zone, "departure_delay" integer)
    LANGUAGE "plpgsql"
    SET "search_path" TO ''
    AS $$
begin
  return query
  select
    stops.name,
    coalesce(
      to_timestamp(stop_time_updates.arrival_date) at time zone 'Europe/Paris',
      service_days.date + stop_times.arrival_seconds * interval '1 second'
    ) as arrival_date,
    stop_time_updates.arrival_delay,
    coalesce(
      to_timestamp(stop_time_updates.departure_date) at time zone 'Europe/Paris',
      service_days.date + stop_times.departure_seconds * interval '1 second'
    ) as departure_date,
    stop_time_updates.departure_delay
  from public.stop_times
  join public.stops on stops.id = stop_times.stop_id
  join public.trips on trips.id = stop_times.trip_id
  join public.service_days on service_days.service_id = trips.service_id
  left join public.stop_time_updates
  on stop_time_updates.trip_id = stop_times.trip_id
  and stop_time_updates.stop_id = stop_times.stop_id
  and stop_time_updates.trip_date = service_days.date
  where stop_times.trip_id = on_trip_id
  and service_days.date = on_date
  order by stop_times.stop_sequence;
end;
$$;


ALTER FUNCTION "public"."get_trip_stops"("on_trip_id" "text", "on_date" "date") OWNER TO "postgres";

SET default_tablespace = '';

SET default_table_access_method = "heap";


CREATE TABLE IF NOT EXISTS "public"."routes" (
    "id" "text" NOT NULL,
    "short_name" "text",
    "type" smallint NOT NULL,
    "color" "text",
    "text_color" "text"
);


ALTER TABLE "public"."routes" OWNER TO "postgres";


CREATE TABLE IF NOT EXISTS "public"."service_days" (
    "service_id" bigint NOT NULL,
    "date" "date" NOT NULL
);


ALTER TABLE "public"."service_days" OWNER TO "postgres";


CREATE TABLE IF NOT EXISTS "public"."services" (
    "id" bigint NOT NULL
);


ALTER TABLE "public"."services" OWNER TO "postgres";


CREATE TABLE IF NOT EXISTS "public"."stations" (
    "id" "text" NOT NULL,
    "name" "text" NOT NULL,
    "latitude" double precision NOT NULL,
    "longitude" double precision NOT NULL
);


ALTER TABLE "public"."stations" OWNER TO "postgres";


CREATE TABLE IF NOT EXISTS "public"."stop_times" (
    "trip_id" "text" NOT NULL,
    "stop_id" "text" NOT NULL,
    "arrival_seconds" integer NOT NULL,
    "departure_seconds" integer NOT NULL,
    "stop_sequence" bigint NOT NULL
);


ALTER TABLE "public"."stop_times" OWNER TO "postgres";


CREATE TABLE IF NOT EXISTS "public"."stops" (
    "id" "text" NOT NULL,
    "name" "text" NOT NULL,
    "latitude" double precision NOT NULL,
    "longitude" double precision NOT NULL,
    "station_id" "text" NOT NULL
);


ALTER TABLE "public"."stops" OWNER TO "postgres";


CREATE TABLE IF NOT EXISTS "public"."trips" (
    "id" "text" NOT NULL,
    "route_id" "text" NOT NULL,
    "service_id" bigint NOT NULL,
    "headsign" "text" NOT NULL,
    "origin_id" "text" NOT NULL,
    "destination_id" "text" NOT NULL
);


ALTER TABLE "public"."trips" OWNER TO "postgres";


ALTER TABLE ONLY "public"."routes"
    ADD CONSTRAINT "routes_pkey" PRIMARY KEY ("id");



ALTER TABLE ONLY "public"."service_days"
    ADD CONSTRAINT "service_days_pkey" PRIMARY KEY ("service_id", "date");



ALTER TABLE ONLY "public"."services"
    ADD CONSTRAINT "services_id_key" UNIQUE ("id");



ALTER TABLE ONLY "public"."services"
    ADD CONSTRAINT "services_pkey" PRIMARY KEY ("id");



ALTER TABLE ONLY "public"."stations"
    ADD CONSTRAINT "stations_pkey" PRIMARY KEY ("id");



ALTER TABLE ONLY "public"."stop_times"
    ADD CONSTRAINT "stop_times_pkey" PRIMARY KEY ("trip_id", "stop_id", "stop_sequence");



ALTER TABLE ONLY "public"."stops"
    ADD CONSTRAINT "stops_pkey" PRIMARY KEY ("id");



ALTER TABLE ONLY "public"."trips"
    ADD CONSTRAINT "trips_pkey" PRIMARY KEY ("id");



ALTER TABLE ONLY "public"."service_days"
    ADD CONSTRAINT "service_days_service_id_fkey" FOREIGN KEY ("service_id") REFERENCES "public"."services"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."stop_times"
    ADD CONSTRAINT "stop_times_stop_id_fkey" FOREIGN KEY ("stop_id") REFERENCES "public"."stops"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."stop_times"
    ADD CONSTRAINT "stop_times_trip_id_fkey" FOREIGN KEY ("trip_id") REFERENCES "public"."trips"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."stops"
    ADD CONSTRAINT "stops_station_id_fkey" FOREIGN KEY ("station_id") REFERENCES "public"."stations"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."trips"
    ADD CONSTRAINT "trips_destination_id_fkey" FOREIGN KEY ("destination_id") REFERENCES "public"."stops"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."trips"
    ADD CONSTRAINT "trips_origin_id_fkey" FOREIGN KEY ("origin_id") REFERENCES "public"."stops"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."trips"
    ADD CONSTRAINT "trips_route_id_fkey" FOREIGN KEY ("route_id") REFERENCES "public"."routes"("id") ON UPDATE CASCADE ON DELETE CASCADE;



ALTER TABLE ONLY "public"."trips"
    ADD CONSTRAINT "trips_service_id_fkey" FOREIGN KEY ("service_id") REFERENCES "public"."services"("id") ON UPDATE CASCADE ON DELETE CASCADE;



CREATE POLICY "Enable read access for all users" ON "public"."routes" FOR SELECT USING (true);



CREATE POLICY "Enable read access for all users" ON "public"."service_days" FOR SELECT USING (true);



CREATE POLICY "Enable read access for all users" ON "public"."services" FOR SELECT USING (true);



CREATE POLICY "Enable read access for all users" ON "public"."stations" FOR SELECT USING (true);



CREATE POLICY "Enable read access for all users" ON "public"."stop_times" FOR SELECT USING (true);



CREATE POLICY "Enable read access for all users" ON "public"."stops" FOR SELECT USING (true);



CREATE POLICY "Enable read access for all users" ON "public"."trips" FOR SELECT USING (true);



ALTER TABLE "public"."routes" ENABLE ROW LEVEL SECURITY;


ALTER TABLE "public"."service_days" ENABLE ROW LEVEL SECURITY;


ALTER TABLE "public"."services" ENABLE ROW LEVEL SECURITY;


ALTER TABLE "public"."stations" ENABLE ROW LEVEL SECURITY;


ALTER TABLE "public"."stop_times" ENABLE ROW LEVEL SECURITY;


ALTER TABLE "public"."stops" ENABLE ROW LEVEL SECURITY;


ALTER TABLE "public"."trips" ENABLE ROW LEVEL SECURITY;




ALTER PUBLICATION "supabase_realtime" OWNER TO "postgres";


GRANT USAGE ON SCHEMA "public" TO "postgres";
GRANT USAGE ON SCHEMA "public" TO "anon";
GRANT USAGE ON SCHEMA "public" TO "authenticated";
GRANT USAGE ON SCHEMA "public" TO "service_role";






















































































































































GRANT ALL ON FUNCTION "public"."get_arrivals"("at_station_id" "text", "on_date" "date", "from_seconds" integer) TO "anon";
GRANT ALL ON FUNCTION "public"."get_arrivals"("at_station_id" "text", "on_date" "date", "from_seconds" integer) TO "authenticated";
GRANT ALL ON FUNCTION "public"."get_arrivals"("at_station_id" "text", "on_date" "date", "from_seconds" integer) TO "service_role";



GRANT ALL ON FUNCTION "public"."get_departures"("at_station_id" "text", "on_date" "date", "from_seconds" integer) TO "anon";
GRANT ALL ON FUNCTION "public"."get_departures"("at_station_id" "text", "on_date" "date", "from_seconds" integer) TO "authenticated";
GRANT ALL ON FUNCTION "public"."get_departures"("at_station_id" "text", "on_date" "date", "from_seconds" integer) TO "service_role";



GRANT ALL ON FUNCTION "public"."get_trip_stops"("on_trip_id" "text", "on_date" "date") TO "anon";
GRANT ALL ON FUNCTION "public"."get_trip_stops"("on_trip_id" "text", "on_date" "date") TO "authenticated";
GRANT ALL ON FUNCTION "public"."get_trip_stops"("on_trip_id" "text", "on_date" "date") TO "service_role";


















GRANT ALL ON TABLE "public"."routes" TO "anon";
GRANT ALL ON TABLE "public"."routes" TO "authenticated";
GRANT ALL ON TABLE "public"."routes" TO "service_role";



GRANT ALL ON TABLE "public"."service_days" TO "anon";
GRANT ALL ON TABLE "public"."service_days" TO "authenticated";
GRANT ALL ON TABLE "public"."service_days" TO "service_role";



GRANT ALL ON TABLE "public"."services" TO "anon";
GRANT ALL ON TABLE "public"."services" TO "authenticated";
GRANT ALL ON TABLE "public"."services" TO "service_role";



GRANT ALL ON TABLE "public"."stations" TO "anon";
GRANT ALL ON TABLE "public"."stations" TO "authenticated";
GRANT ALL ON TABLE "public"."stations" TO "service_role";



GRANT ALL ON TABLE "public"."stop_times" TO "anon";
GRANT ALL ON TABLE "public"."stop_times" TO "authenticated";
GRANT ALL ON TABLE "public"."stop_times" TO "service_role";



GRANT ALL ON TABLE "public"."stops" TO "anon";
GRANT ALL ON TABLE "public"."stops" TO "authenticated";
GRANT ALL ON TABLE "public"."stops" TO "service_role";



GRANT ALL ON TABLE "public"."trips" TO "anon";
GRANT ALL ON TABLE "public"."trips" TO "authenticated";
GRANT ALL ON TABLE "public"."trips" TO "service_role";









ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON SEQUENCES TO "postgres";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON SEQUENCES TO "anon";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON SEQUENCES TO "authenticated";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON SEQUENCES TO "service_role";






ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON FUNCTIONS TO "postgres";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON FUNCTIONS TO "anon";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON FUNCTIONS TO "authenticated";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON FUNCTIONS TO "service_role";






ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON TABLES TO "postgres";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON TABLES TO "anon";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON TABLES TO "authenticated";
ALTER DEFAULT PRIVILEGES FOR ROLE "postgres" IN SCHEMA "public" GRANT ALL ON TABLES TO "service_role";































drop extension if exists "pg_net";


