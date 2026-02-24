//
//  Colors.swift
//  Traintables
//
//  Created by Jules on 27/12/2025.
//

import Foundation
import SwiftUI

extension Color {
    init(hex: String) {
        var rgb: UInt64 = 0
        Scanner(string: hex).scanHexInt64(&rgb)
        let red = CGFloat((rgb & 0xff00000) >> 16) / 0xff
        let green = CGFloat((rgb & 0xff00) >> 8) / 0xff
        let blue = CGFloat(rgb & 0xff) / 0xff
        self.init(red: red, green: green, blue: blue)
    }
}
