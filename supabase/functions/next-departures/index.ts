import { createClient, SupabaseClient } from "@supabase/supabase-js";
import GtfsRealtimeBindings from "gtfs-realtime-bindings";

interface ScheduledDeparture {
    trip_id: string;
    route_short_name: string;
    destination: string;
    departure_time: Date;
    headsign: string;
    color: string;
    text_color: string;
}

interface DepartureUpdate {
    trip_id: string;
    stop_id: string;
    trip_date: string;
    departure_delay: number | undefined;
    departure_date: number | undefined;
}

async function get_scheduled_departures(
    supabaseClient: SupabaseClient,
    station_id: string,
) {
    const now = new Date();
    const date = now.toISOString().split("T")[0];
    const seconds =
        now.getHours() * 3600 + now.getMinutes() * 60 + now.getSeconds();

    const { data, error } = await supabaseClient.rpc("get_departures", {
        at_station_id: station_id,
        on_date: date,
        from_seconds: seconds,
    });

    return data as ScheduledDeparture[];
}

async function get_stops_id(
    supabaseClient: SupabaseClient,
    station_id: string,
) {
    const { data } = await supabaseClient
        .from("stops")
        .select("id")
        .eq("station_id", station_id);
    return data!.map((stop) => stop.id);
}

async function get_departure_updates(stops_id: string[]) {
    const response = await fetch(
        "https://proxy.transport.data.gouv.fr/resource/sncf-gtfs-rt-trip-updates",
    );
    const buffer = await response.arrayBuffer();

    const feed = GtfsRealtimeBindings.transit_realtime.FeedMessage.decode(
        new Uint8Array(buffer),
    );

    const departureUpdates: DepartureUpdate[] = [];

    feed.entity.forEach((entity) => {
        if (entity.tripUpdate) {
            const tripUpdate = entity.tripUpdate;

            for (const stopTimeUpdate of tripUpdate.stopTimeUpdate!) {
                if (
                    stops_id.includes(stopTimeUpdate.stopId!) &&
                    stopTimeUpdate.departure != null
                ) {
                    departureUpdates.push({
                        trip_id: tripUpdate.trip.tripId!,
                        stop_id: stopTimeUpdate.stopId!,
                        trip_date: tripUpdate.trip.startDate!,
                        departure_delay: stopTimeUpdate.departure.delay!,
                        departure_date: Number(
                            stopTimeUpdate.departure.time!.toString(),
                        ),
                    });
                }
            }
        }
    });

    return departureUpdates;
}

Deno.serve(async (req) => {
    const { station_id } = await req.json();
    const supabaseClient = createClient(
        Deno.env.get("SUPABASE_URL")!,
        Deno.env.get("SUPABASE_SERVICE_ROLE_KEY")!,
    );
    const scheduled_departures = await get_scheduled_departures(
        supabaseClient,
        station_id,
    );
    const stops_id = await get_stops_id(supabaseClient, station_id);
    const departure_updates = await get_departure_updates(stops_id);
    return new Response(JSON.stringify(departure_updates, null, 2));
});

/* To invoke locally:

  1. Run `supabase start` (see: https://supabase.com/docs/reference/cli/supabase-start)
  2. Make an HTTP request:

  curl -i --location --request POST 'http://127.0.0.1:54321/functions/v1/next-departures' \
    --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0' \
    --header 'Content-Type: application/json' \
    --data '{"name":"Functions"}'

*/
