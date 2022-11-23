//
//  AlbumifyApp.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

@main
struct AlbumifyApp: App {
    @StateObject var spotify = Spotify()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(spotify)
                .onAppear(){
                    spotify.login(username: ProcessInfo.processInfo.environment["SPOTIFY_USERNAME"]!, password: ProcessInfo.processInfo.environment["SPOTIFY_PASSWORD"]!)
                }
        }
    }
}
