//
//  Trip.swift
//  Traintables
//
//  Created by Jules on 25/12/2025.
//

import Foundation
import SQLite3

struct TripStop: Decodable, Hashable {
    let name: String
    let arrivalDate: Date
//    let arrivalDelay: Int32?
    let departureDate: Date
//    let departureDelay: Int32?
    
    enum CodingKeys: String, CodingKey {
        case name
        case arrivalDate = "arrival_date"
//        case arrivalDelay = "arrival_delay"
        case departureDate = "departure_date"
//        case departureDelay = "departure_delay"
    }
    
    init(from decoder: any Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.name = try container.decode(String.self, forKey: .name)
        let dateFormatter = DateFormatter()
        dateFormatter.timeZone = TimeZone(identifier: "Europe/Paris")
        dateFormatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss"
        let arrivalDateString = try container.decode(String.self, forKey: .arrivalDate)
        self.arrivalDate = dateFormatter.date(from: arrivalDateString)!
//        self.arrivalDelay = try container.decodeIfPresent(Int32.self, forKey: .arrivalDelay)
        let departureDateString = try container.decode(String.self, forKey: .departureDate)
        self.departureDate = dateFormatter.date(from: departureDateString)!
//        self.departureDelay = try container.decodeIfPresent(Int32.self, forKey: .departureDelay)
    }
}
