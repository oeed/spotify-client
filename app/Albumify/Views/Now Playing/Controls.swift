//
//  Controls.swift
//  Albumify
//
//  Created by Oliver Cooper on 13/11/22.
//

import SwiftUI

struct Controls: View {
    @EnvironmentObject var spotify: Spotify
    
    var body: some View {
        HStack(alignment: .center, spacing: 0) {
            ControlButton(
                imageName: "backward.fill",
                isPrimary: false,
                action: {
                    spotify.session?.previousTrack()
                }
            )
            
            ControlButton(
                imageName: "play.fill",
                isPrimary: true,
                action: {
                    spotify.session?.pause()
                }
            )
            
            ControlButton(
                imageName: "forward.fill",
                isPrimary: false,
                action: {
                    spotify.session?.nextTrack()
                }
            )
        }
    }
}

struct ControlButton: View {
    let imageName: String
    let isPrimary: Bool
    let action: () -> Void
    
    var body: some View {
        Button(action: action, label: {
            Image(systemName: imageName)
                .resizable()
                .aspectRatio(contentMode: .fit)
                .padding(.vertical, isPrimary ? 3 : 6)
        })
    }
}

struct Controls_Previews: PreviewProvider {
    static var previews: some View {
        Controls()
    }
}
