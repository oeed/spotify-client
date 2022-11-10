//
//  ArtistListSection.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct ArtistListSection: View {
    let index: Int
    @State private var isPinned: Bool = false
    @State private var headerHeight: CGFloat? = nil
    
    var body: some View {
        Section(header:
            ArtistListHeader(name: "Artist \(index)", isPinned: isPinned)
            .background(GeometryReader { geometry in
                // detect current height of the header
                Color.clear.onAppear {
                    self.headerHeight = geometry.size.height;
                }
            })
        ) {
            AlbumList(count:1).background(GeometryReader { geometry in
                // detect current position of header
                Color.clear.preference(key: ViewOffsetKeyBool.self,
                                       value: true)
            })
        }
//
//                    .onPreferenceChange(HeaderHeightKey.self) {
////                        let headerHeight = self.preference(value: HeaderHeightKey.self)
//
//                        print("header change \(self.headerHeight) \($0)")
//                    }
        .onPreferenceChange(ViewOffsetKeyBool.self) {
            
            print("is pinned \(index) \($0)")
            // verify if position is zero (pinned) in container coordinates
//                        if $0 {
//                            self.pinned = index
//                            print("set pinned \(self.pinned)")
//                        }
        }
    }
}

struct ArtistListSection_Previews: PreviewProvider {
    static var previews: some View {
        ArtistListSection(index: 1)
    }
}


func isHeaderPinned(_ list: GeometryProxy, _ headerHeight: CGFloat?) -> Bool {

//    print("area \(list.frame(in: .named("area")))")
//    print("scroll \(list.frame(in: .named("scroll")))")
//    print("headerHeight \(headerHeight)")

//    if let headerHeight = headerHeight {
////        print("is \(list.frame(in: .named("scroll")).origin.y < headerHeight)")
//        return list.frame(in: .named("scroll")).origin.y < headerHeight
//    }
//    else {
//        return false
//    }
    return true
}

/// A preference key to store ScrollView offset
struct ViewOffsetKey: PreferenceKey {
    typealias Value = Int?
    static var defaultValue = Optional<Int>.none
    static func reduce(value: inout Value, nextValue: () -> Value) {
        if let nextValue = nextValue() {
            value = nextValue
        }
    }
}

/// A preference key to store ScrollView offset
struct ViewOffsetKeyBool: PreferenceKey {
    typealias Value = Bool
    static var defaultValue = false
    static func reduce(value: inout Value, nextValue: () -> Value) {
        print("reduce",nextValue())
        value = nextValue()
    }
}

//
///// A preference key to store ScrollView offset
//struct HeaderHeightKey: PreferenceKey {
//    typealias Value = CGFloat
//    static var defaultValue = -1 as CGFloat //CGFloat.zero
//    static func reduce(value: inout Value, nextValue: () -> Value) {
//        print("reduce height",value, nextValue())
//        value = nextValue()
//    }
//}
