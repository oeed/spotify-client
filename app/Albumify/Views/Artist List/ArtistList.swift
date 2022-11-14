//
//  ArtistList.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct ArtistList: View {
    let data = (1...100).map { "Artist \($0)" }
    @State private var pinned: Int? = nil
    @State private var headerHeight: CGFloat? = nil
    
    var body: some View {
        ScrollView {
            ZStack {
                LazyVStack(spacing: 0, pinnedViews: [.sectionHeaders]) {
                    ForEach(1..<20) { index in
                        Section(header:
                                    ArtistListHeader(name: "Artist \(index)", isPinned: index == pinned)
                        ) {
                            AlbumList(count: index)
                        }
                    }
                }
            }
        }
        .coordinateSpace(name: "scroll")
        .background(.background)
    }
}
//
//func isPinned(_ list: GeometryProxy, _ headerHeight: CGFloat?) -> Bool {
//
////    print("area \(list.frame(in: .named("area")))")
////    print("scroll \(list.frame(in: .named("scroll")))")
////    print("headerHeight \(headerHeight)")
//
//    if let headerHeight = headerHeight {
////        print("is \(list.frame(in: .named("scroll")).origin.y < headerHeight)")
//        return list.frame(in: .named("scroll")).origin.y < headerHeight
//    }
//    else {
//        return false
//    }
//}

struct ArtistList_Previews: PreviewProvider {
    static var previews: some View {
        ArtistList().frame(width: 800, height: 500)
    }
}
