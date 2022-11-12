//
//  AlbumButton.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct NowPlayingAlbum: View {
    @Environment(\.pixelLength) var pixelLength: CGFloat

    var body: some View {
        ZStack {
            Image("1")
                .resizable()
                .aspectRatio(contentMode: .fit)
        }
        .clipShape(RoundedRectangle(cornerRadius: 4))
        .overlay {
            RoundedRectangle(cornerRadius: 4).inset(by: pixelLength / 2).stroke(.separator, lineWidth: pixelLength)
        }
    }
}

struct NowPlayingAlbum_Previews: PreviewProvider {
    static var previews: some View {
        NowPlayingAlbum()
            .frame(width: 32, height: 32)
    }
}
