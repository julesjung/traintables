//
//  Arrival.swift
//  Traintables
//
//  Created by Jules on 25/12/2025.
//

import Foundation
import SQLite3

struct Arrival: Decodable, Hashable {
    let tripId: String
    let routeShortName: String?
    let origin: String
    let arrivalDate: Date
//    let arrivalDelay: Int32?
    let headsign: String
    let color: String?
    let textColor: String?
    
    enum CodingKeys: String, CodingKey {
        case tripId = "trip_id"
        case routeShortName = "route_short_name"
        case routeLongName = "route_long_name"
        case origin
        case arrivalDate = "arrival_time"
//        case arrivalDelay = "arrival_delay"
        case headsign = "headsign"
        case color
        case textColor = "text_color"
    }
    
    init(from decoder: any Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.tripId = try container.decode(String.self, forKey: .tripId)
        self.origin = try container.decode(String.self, forKey: .origin)
        self.routeShortName = try container.decodeIfPresent(String.self, forKey: .routeShortName)
        let arrivalDateString = try container.decode(String.self, forKey: .arrivalDate)
        let dateFormatter = DateFormatter()
        dateFormatter.timeZone = TimeZone(identifier: "Europe/Paris")
        dateFormatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss"
        self.arrivalDate = dateFormatter.date(from: arrivalDateString)!
//        self.arrivalDelay = try container.decodeIfPresent(Int32.self, forKey: .arrivalDelay)
        self.headsign = try container.decode(String.self, forKey: .headsign)
        self.color = try container.decodeIfPresent(String.self, forKey: .color)
        self.textColor = try container.decodeIfPresent(String.self, forKey: .textColor)
    }
}
