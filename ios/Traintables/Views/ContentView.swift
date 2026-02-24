//
//  ContentView.swift
//  Traintables
//
//  Created by Jules on 21/12/2025.
//

import SwiftUI

struct ContentView: View {    
    var body: some View {
        TabView {
            Tab("home", systemImage: "house") {
                
            }
            Tab(role: .search) {
                SearchView()
            }
        }
    }
}
