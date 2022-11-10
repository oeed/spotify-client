//
//  AlbumProgressIndicator.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct AlbumProgressIndicator: View {
    @Environment(\.pixelLength) var pixelLength: CGFloat
    
    var body: some View {
        GeometryReader { metrics in
            ZStack(alignment: .topLeading) {
                Rectangle()
                    .fill(.separator)
                    .frame(height: pixelLength)

                Color.secondary
                    .frame(width: 0.4 * metrics.size.width)
                
                Rectangle()
                    .fill(Color(NSColor.windowBackgroundColor))
                    .frame(width: 1)
                    .offset(x: 0.3 * metrics.size.width)
                    
                
                Rectangle()
                    .fill(.tertiary)
                    .frame(width: 1)
                    .offset(x: 0.8 * metrics.size.width)
            }
        }
        .frame(height: 3)
    }
}

struct AlbumProgressIndicator_Previews: PreviewProvider {
    static var previews: some View {
        AlbumProgressIndicator()
    }
}
