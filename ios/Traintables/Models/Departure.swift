//
//  Departure.swift
//  Traintables
//
//  Created by Jules on 24/12/2025.
//

import Foundation
import SQLite3

struct Departure: Decodable, Hashable {
    let tripId: String
    let routeShortName: String?
    let destination: String
    let departureDate: Date
//    let departureDelay: Int32?
    let headsign: String
    let color: String?
    let textColor: String?
    
    enum CodingKeys: String, CodingKey {
        case tripId = "trip_id"
        case routeShortName = "route_short_name"
        case destination
        case departureDate = "departure_time"
//        case departureDelay = "departure_delay"
        case headsign = "headsign"
        case color
        case textColor = "text_color"
    }
    
    init(from decoder: any Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.tripId = try container.decode(String.self, forKey: .tripId)
        self.destination = try container.decode(String.self, forKey: .destination)
        self.routeShortName = try container.decodeIfPresent(String.self, forKey: .routeShortName)
        let departureDateString = try container.decode(String.self, forKey: .departureDate)
        let dateFormatter = DateFormatter()
        dateFormatter.timeZone = TimeZone(identifier: "Europe/Paris")
        dateFormatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss"
        self.departureDate = dateFormatter.date(from: departureDateString)!
//        self.departureDelay = try container.decodeIfPresent(Int32.self, forKey: .departureDelay)
        self.headsign = try container.decode(String.self, forKey: .headsign)
        self.color = try container.decodeIfPresent(String.self, forKey: .color)
        self.textColor = try container.decodeIfPresent(String.self, forKey: .textColor)
    }
}
