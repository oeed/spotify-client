//
//  NowPlaying.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct NowPlaying: View {
    @Environment(\.pixelLength) var pixelLength: CGFloat
    @State private var isHovering: Bool = false
    
    var body: some View {
        
        HStack {
            NowPlayingAlbum()

            VStack(spacing: 0) {
                ZStack {
                    HStack(alignment: .center) {
                        Text("Magma")
                            .font(.headline)
                            .foregroundColor(.primary)
                            .padding(.leading)
                        Text("Ice, Death, Planets, Lungs, Mushrooms and Lava")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                            .lineLimit(1)
                            .padding(.trailing)
                            .truncationMode(/*@START_MENU_TOKEN@*/.tail/*@END_MENU_TOKEN@*/)
                        
                    }
                    .opacity(isHovering ? 0 : 1)
                    .animation(.easeInOut(duration: 0.15), value: isHovering)
                    
                    Controls()
                        .padding(.vertical, 2)
                    .offset(y: 0.5)
                    .opacity(isHovering ? 1 : 0)
                    .animation(.easeInOut(duration: 0.15), value: isHovering)
                }
                .frame(maxHeight: .infinity)
                AlbumProgressIndicator()
            }
            .background(.ultraThinMaterial)
            .clipShape(RoundedRectangle(cornerRadius: 4))
            .overlay {
                RoundedRectangle(cornerRadius: 4).inset(by: pixelLength / 2).stroke(.separator, lineWidth: pixelLength)
            }
        }
        .onHover { over in
            isHovering = over
        }
        
    }
}

struct NowPlaying_Previews: PreviewProvider {
    static var previews: some View {
        NowPlaying()
            .frame(width: 500, height: 32)
    }
}
