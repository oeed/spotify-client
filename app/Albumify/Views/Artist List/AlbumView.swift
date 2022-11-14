//
//  AlbumView.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct AlbumView: View {
    let image: Image
    @State private var isHovering: Bool = false
    @State private var isPressing: Bool = false
    
    init() {
        self.init(image: Image("\(Int.random(in: 1...13))"))
    }
    
    init(image: Image) {
        self.image = image
    }
    
    var body: some View {
        self.image
            .resizable()
            .aspectRatio(contentMode: .fit)
            .clipShape(RoundedRectangle(cornerRadius: 6))
            .shadow(color: Color(white: 0, opacity: 0.2), radius: 2, y: 1)
            .overlay {
                RoundedRectangle(cornerRadius: 6).inset(by: 1 / 2).stroke(.separator, lineWidth: 1)
            }
            .scaleEffect(isPressing ? 0.98 : isHovering ? 1.02 : 1)
            .animation(.easeInOut(duration: 0.08), value: [isHovering, isPressing])
            .onHover { over in
                isHovering = over
            }
            .simultaneousGesture(
                DragGesture(minimumDistance: 0)
                    .onChanged({ _ in
                        self.isPressing = true
                    })
                    .onEnded({ _ in
                        self.isPressing = false
                    })
            )
    }
}

struct AlbumView_Previews: PreviewProvider {
    static var previews: some View {
        AlbumView(image: Image("7"))
            .frame(width: 160, height: 160)
    }
}
