//
//  TripView.swift
//  Traintables
//
//  Created by Jules on 25/12/2025.
//

import SwiftUI
import Supabase
import PostgREST
import MapKit

struct TripView: View {
    let tableItem: TableItem
    @State private var stops: [TripStop]?
    
    var body: some View {
        VStack {
            if stops != nil {
                ScrollView {
                    timeline
                        .padding()
                }
            } else {
                Spacer()
                ProgressView("loading_trip_stops")
                Spacer()
            }
        }
        .navigationTitle("route_\(tableItem.headsign)")
        .task {
            if stops == nil {
                let dateFormatter = ISO8601DateFormatter()
                dateFormatter.formatOptions = [.withFullDate]
                
                stops = try! await supabaseClient.rpc("get_trip_stops", params: ["on_trip_id": tableItem.tripId, "on_date": dateFormatter.string(from: tableItem.date)]).execute().value
            }
        }
    }
    
    var timeline: some View {
        GroupBox {
            VStack(spacing: 0) {
                ForEach(stops!.enumerated(), id: \.element) { index, stop in
                    HStack(spacing: 12) {
                        VStack {
                            if index == 0 {
                                Text(stop.departureDate.formatted(date: .omitted, time: .shortened))
                                    .monospaced()
                                    .bold()
//                                    .foregroundStyle(stop.departureDelay != nil ? (stop.departureDelay! > 0 ? .orange : .green) : .gray)
                                    .foregroundStyle(.gray)
                            } else if index == stops!.count - 1 {
                                Text(stop.arrivalDate.formatted(date: .omitted, time: .shortened))
                                    .monospaced()
                                    .bold()
//                                    .foregroundStyle(stop.arrivalDelay != nil ? (stop.arrivalDelay! > 0 ? .orange : .green) : .gray)
                                    .foregroundStyle(.gray)
                            } else {
                                Text(stop.arrivalDate.formatted(date: .omitted, time: .shortened))
                                    .monospaced()
                                    .bold()
//                                    .foregroundStyle(stop.arrivalDelay != nil ? (stop.arrivalDelay! > 0 ? .orange : .green) : .gray)
                                    .foregroundStyle(.gray)
                                Text(stop.departureDate.formatted(date: .omitted, time: .shortened))
                                    .monospaced()
                                    .bold()
//                                    .foregroundStyle(stop.departureDelay != nil ? (stop.departureDelay! > 0 ? .orange : .green) : .gray)
                                    .foregroundStyle(.gray)
                            }
                        }
                        
                        ZStack(alignment: .leading) {
                            if index != 0 {
                                Rectangle()
                                    .fill(Date() > stop.arrivalDate ? tableItem.color : .gray)
                                    .frame(width: 18)
                                    .frame(height: 32)
                                    .offset(y: -16)
                            }
                            
                            Circle()
                                .fill(tableItem.textColor)
                                .stroke(Date() > stop.arrivalDate ? tableItem.color : .gray, lineWidth: 2)
                                .frame(width: 16, height: 16)
                                .offset(x: 1)
                                .zIndex(1)
                            
                            if index != stops!.count - 1 {
                                Rectangle()
                                    .fill(Date() > stop.departureDate ? tableItem.color : .gray)
                                    .frame(width: 18)
                                    .frame(height: 32)
                                    .offset(y: 16)
                            }
                        }
                        .frame(height: index != 0 && index != stops!.count - 1 ? 64 : 40)
                        .clipped()
                        
                        VStack(alignment: .leading) {
                            Text(stop.name)
                                .lineLimit(1)
//                            if tableItem.delay != nil {
//                                DelayIndicator(delay: tableItem.delay!)
//                            }
                        }
                        
                        Spacer()
                    }
                }
            }
        }
    }
}
