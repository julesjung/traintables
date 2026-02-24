//
//  StationView.swift
//  Traintables
//
//  Created by Jules on 23/12/2025.
//

import SwiftUI
import Supabase
import PostgREST

struct StationView: View {
    let station: Station
    @State private var departures: Dictionary<Date, [Departure]>?
    @State private var arrivals: Dictionary<Date, [Arrival]>?
    @State private var selectedPage: Page = .departurePage
    
    var sortedDepartures: [(key: Date, value: [Departure])]? {
        departures?.sorted { $0.key < $1.key }
    }
    
    var sortedArrivals: [(key: Date, value: [Arrival])]? {
        arrivals?.sorted { $0.key < $1.key }
    }
    
    var body: some View {
        VStack {
            Picker("page_selection", selection: $selectedPage) {
                Text("departures").tag(Page.departurePage)
                Text("arrivals").tag(Page.arrivalPage)
            }
            .pickerStyle(.segmented)
            .padding()
            switch selectedPage {
            case .departurePage:
                if departures == nil {
                    Spacer()
                    ProgressView("loading_departures")
                    Spacer()
                } else {
                    if departures!.isEmpty {
                        Spacer()
                        Text("no_departure_found")
                        Spacer()
                    } else {
                        departuresTab
                    }
                }
            case .arrivalPage:
                if arrivals == nil {
                    Spacer()
                    ProgressView("loading_arrivals")
                    Spacer()
                } else {
                    if arrivals!.isEmpty {
                        Spacer()
                        Text("no_arrival_found")
                        Spacer()
                    } else {
                        arrivalsTab
                    }
                }
            }
        }
        .navigationTitle(station.name)
        .navigationBarTitleDisplayMode(.inline)
        .task {
            if departures == nil && selectedPage == .departurePage {
                await loadDeparturesTable()
            }
        }
        .onChange(of: selectedPage) {
            Task {
                if departures == nil && selectedPage == .departurePage {
                    await loadDeparturesTable()
                } else if arrivals == nil && selectedPage == .arrivalPage {
                    await loadArrivalsTable()
                }
            }
        }
        .refreshable {
            if selectedPage == .departurePage {
                await loadDeparturesTable()
            } else {
                await loadArrivalsTable()
            }
        }
    }
        
    var departuresTab: some View {
        List {
            ForEach(sortedDepartures!, id: \.key) { date, dateDepartures in
                Section(date.formatted(date: .long, time: .omitted)) {
                    displayDailyDepartures(departures: dateDepartures)
                }
            }
        }
        .listStyle(.plain)
    }
    
    var arrivalsTab: some View {
        List {
            ForEach(sortedArrivals!, id: \.key) { date, dateArrivals in
                Section(date.formatted(date: .long, time: .omitted)) {
                    displayDailyArrivals(arrivals: dateArrivals)
                }
            }
        }
        .listStyle(.plain)
    }
    
    @ViewBuilder
    func displayDailyDepartures(departures: [Departure]) -> some View {
        ForEach(departures, id: \.self) { departure in
            StationTableItem(tableItem: TableItem(
                tripId: departure.tripId,
                name: departure.destination,
                shortName: departure.routeShortName,
                date: departure.departureDate,
//                delay: departure.departureDelay,
                headsign: departure.headsign,
                color: departure.color != nil ? Color(hex: departure.color!) : .accentColor,
                textColor: departure.textColor != nil ? Color(hex: departure.textColor!) : .white
            ))
        }
    }
    
    @ViewBuilder
    func displayDailyArrivals(arrivals: [Arrival]) -> some View {
        ForEach(arrivals, id: \.self) { arrival in
            StationTableItem(tableItem: TableItem(
                tripId: arrival.tripId,
                name: arrival.origin,
                shortName: arrival.routeShortName,
                date: arrival.arrivalDate,
//                delay: arrival.arrivalDelay,
                headsign: arrival.headsign,
                color: arrival.color != nil ? Color(hex: arrival.color!) : .accentColor,
                textColor: arrival.textColor != nil ? Color(hex: arrival.textColor!) : .white
            ))
        }
    }
    
    func loadDeparturesTable() async {
        let calendar = Calendar(identifier: .gregorian)

        let now = Date()
        let startOfDay = calendar.startOfDay(for: now)
        let secondsSinceMidnight = Int(now.timeIntervalSince(startOfDay))

        let dateFormatter = ISO8601DateFormatter()
        dateFormatter.formatOptions = [.withFullDate]
        
        print([
            "at_station_id": station.id,
            "on_date": dateFormatter.string(from: now),
            "from_seconds": "\(secondsSinceMidnight)"
        ])
        
        let ungroupedDepartures: [Departure] = try! await supabaseClient.rpc("get_departures", params: [
            "at_station_id": station.id,
            "on_date": dateFormatter.string(from: now),
            "from_seconds": "\(secondsSinceMidnight)"
        ]).execute().value
        
        departures = Dictionary(grouping: ungroupedDepartures, by: { departure in
            calendar.startOfDay(for: departure.departureDate)
        })
    }
    
    func loadArrivalsTable() async {
        let calendar = Calendar(identifier: .gregorian)

        let now = Date()
        let startOfDay = calendar.startOfDay(for: now)
        let secondsSinceMidnight = Int(now.timeIntervalSince(startOfDay))

        let dateFormatter = ISO8601DateFormatter()
        dateFormatter.formatOptions = [.withFullDate]
        
        let ungroupedArrivals: [Arrival] = try! await supabaseClient.rpc("get_arrivals", params: [
            "at_station_id": station.id,
            "on_date": dateFormatter.string(from: now),
            "from_seconds": "\(secondsSinceMidnight)"
        ]).execute().value
        
        arrivals = Dictionary(grouping: ungroupedArrivals, by: { arrival in
            calendar.startOfDay(for: arrival.arrivalDate)
        })
    }
    
    enum Page {
        case departurePage
        case arrivalPage
    }
}
