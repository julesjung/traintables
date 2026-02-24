//
//  TraintablesApp.swift
//  Traintables
//
//  Created by Jules on 21/12/2025.
//

import SwiftUI
import Supabase

let supabaseClient = SupabaseClient(
    supabaseURL: URL(string: "https://ifeuhrjcyxcxekfhstle.supabase.co")!,
    supabaseKey: "sb_publishable_DyUERwf_Ge5oduPhRlEY9g_h0N2fmbw",
)

@main
struct TraintablesApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
