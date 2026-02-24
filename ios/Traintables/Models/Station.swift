//
//  Station.swift
//  Traintables
//
//  Created by Jules on 23/12/2025.
//

import Foundation
import SQLite3

struct Station: Identifiable {
    let id: String
    let name: String
    let latitude: Double
    let longitude: Double
}
