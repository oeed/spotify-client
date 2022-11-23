//
//  Spotify.swift
//  Albumify
//
//  Created by Oliver Cooper on 23/11/22.
//

import Foundation

class Spotify: ObservableObject {
    @Published var session: Session? = .none
    
    func login(username: String, password: String) {
        session = Session(username, password)
    }
}
