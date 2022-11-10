//
//  ArtistListHeader.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct ArtistListHeader: View {
    let name: String
    var isPinned: Bool
    
    var body: some View {
        HStack {
            Text(name)
                .font(.title)
                .fontWeight(.bold)
            Spacer()
            Image(systemName: "sun.max.fill")
        }
        .padding(.horizontal)
        .padding(.vertical, 10)
        .background(VisualEffectView().shadow(radius: isPinned ? 1 : 0).mask(Rectangle().padding(.bottom, -10)))
    }
}

struct ArtistListHeader_Previews: PreviewProvider {
    static var previews: some View {
        ArtistListHeader(name: "Artist Name", isPinned: false)
    }
}

struct VisualEffectView: NSViewRepresentable {
    var material: NSVisualEffectView.Material = .headerView
    var blendingMode: NSVisualEffectView.BlendingMode = .withinWindow
    
    func makeNSView(context: Context) -> NSVisualEffectView {
        let view = NSVisualEffectView()
        view.material = material;
        view.blendingMode = blendingMode;
        return view;
    }
    
    func updateNSView(_ view: NSVisualEffectView, context: Context) {
        view.material = material;
        view.blendingMode = blendingMode;
    }
}
