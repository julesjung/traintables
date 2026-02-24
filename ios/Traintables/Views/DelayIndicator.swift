//
//  DelayIndicator.swift
//  Traintables
//
//  Created by Jules on 29/12/2025.
//

import SwiftUI

struct DelayIndicator: View {
    let delayString: String
    let delayColor: Color
    
    init(delay: Int32?) {
        if delay! == 0 {
            delayColor = .green
            delayString = String(localized: "on_time")
        } else {
            let absoluteDelay = abs(delay!)
            let formatter = DateComponentsFormatter()
            formatter.allowedUnits = [.hour, .minute]
            formatter.unitsStyle = .short
            
            let duration = formatter.string(from: TimeInterval(absoluteDelay))!
            
            if delay! > 0 {
                delayColor = .orange
                delayString = String(format: NSLocalizedString("late", comment: "The train is late"), duration)
            } else {
                delayColor = .green
                delayString = String(format: NSLocalizedString("early", comment: "The train is early"), duration)
            }
        }
    }
        
        
    var body: some View {
        Text(delayString)
            .font(.footnote)
            .foregroundStyle(delayColor)
    }
}

#Preview {
    DelayIndicator(delay: 600)
}
