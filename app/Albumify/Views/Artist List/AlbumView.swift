//
//  AlbumView.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct AlbumView: View {
    var body: some View {
        Image("magma")
            .resizable()
            .aspectRatio(contentMode: .fit)
            .clipShape(RoundedRectangle(cornerRadius: 4))
            .overlay {
                RoundedRectangle(cornerRadius: 4).inset(by: 1 / 2).stroke(.separator, lineWidth: 1)
            }
            .shadow(color: Color(white: 0, opacity: 0.2), radius: 1, y: 1)
    }
}

struct AlbumView_Previews: PreviewProvider {
    static var previews: some View {
        AlbumView()
            .frame(width: 160, height: 160)
    }
}
