//
//  StationTableItem.swift
//  Traintables
//
//  Created by Jules on 27/12/2025.
//

import SwiftUI

struct TableItem: Hashable {
    let tripId: String
    let name: String
    let shortName: String?
    let date: Date
//    let delay: Int32?
    let headsign: String
    let color: Color
    let textColor: Color
}

struct StationTableItem: View {
    let tableItem: TableItem
    
    var body: some View {
        NavigationLink {
            TripView(tableItem: tableItem)
        } label: {
            HStack(spacing: 12) {
                Text(tableItem.date.formatted(date: .omitted, time: .shortened))
                    .monospaced()
                    .bold()
//                    .foregroundStyle(tableItem.delay != nil ? tableItem.delay! > 0 ? .orange : .green : .gray)
                    .foregroundStyle(.gray)
                VStack(alignment: .leading) {
                    HStack(spacing: 12) {
                        Text(tableItem.name)
                            .lineLimit(1)
                    }
//                    if tableItem.delay != nil {
//                        DelayIndicator(delay: tableItem.delay!)
//                    }
                }
                Spacer()
                if tableItem.shortName != nil {
                    Text(tableItem.shortName!)
                        .foregroundStyle(tableItem.textColor)
                        .font(.footnote)
                        .bold()
                        .padding(.horizontal, 6)
                        .padding(.vertical, 3)
                        .background(tableItem.color, in: Capsule())
                }
            }
        }
    }
}
