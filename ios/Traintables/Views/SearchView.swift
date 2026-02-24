//
//  SearchView.swift
//  Traintables
//
//  Created by Jules on 23/12/2025.
//

import SwiftUI

struct SearchView: View {
    @State private var query: String = ""
    
    private var autocompletions: [Station] = [
        Station(id: "StopArea:OCE87391003", name: "Paris Montparnasse Hall 1 - 2", latitude: 48.841172, longitude: 2.320514),
        Station(id: "StopArea:OCE87471003", name: "Rennes", latitude: 48.103517, longitude: -1.672744)
    ]
    
    var body: some View {
        NavigationView {
            List(autocompletions) { autocompletion in
                NavigationLink {
                    StationView(station: autocompletion)
                } label: {
                    Text(autocompletion.name)
                }
            }
            .listStyle(.plain)
            .navigationTitle("search")
        }
        .searchable(text: $query)
    }
}
