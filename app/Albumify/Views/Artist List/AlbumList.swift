//
//  AlbumList.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct AlbumList: View {
//    let data = (1...10).map { "Album \($0)" }
    let count: Int
    
    let columns = [
        GridItem(.adaptive(minimum: 140), spacing: 20)
    ]
    
    var body: some View {
        LazyVGrid(columns: columns, spacing: 20) {
            ForEach(1...Int.random(in: 1...10), id: \.self) { item in
                AlbumView()
            }
        }
        .padding([.leading, .bottom, .trailing])
    }
}

struct AlbumList_Previews: PreviewProvider {
    static var previews: some View {
        AlbumList(count: 3)
            .frame(width: 500)
    }
}
