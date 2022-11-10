//
//  NowPlaying.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct NowPlaying: View {
    @Environment(\.pixelLength) var pixelLength: CGFloat
    
    var body: some View {
        
        HStack {
            Image("magma")
                .resizable()
                .aspectRatio(contentMode: .fit)
                .clipShape(RoundedRectangle(cornerRadius: 4))
                .overlay {
                    RoundedRectangle(cornerRadius: 4).inset(by: pixelLength / 2).stroke(.separator, lineWidth: pixelLength)
                }

            VStack(spacing: 0) {
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
                .frame(maxHeight: .infinity)
                AlbumProgressIndicator()
            }
            .background(.ultraThinMaterial)
            .clipShape(RoundedRectangle(cornerRadius: 4))
            .overlay {
                RoundedRectangle(cornerRadius: 4).inset(by: pixelLength / 2).stroke(.separator, lineWidth: pixelLength)
            }
        }
        
    }
}

struct NowPlaying_Previews: PreviewProvider {
    static var previews: some View {
        NowPlaying()
            .frame(width: 500, height: 32)
    }
}
