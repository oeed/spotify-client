//
//  ContentView.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct ContentView: View {
    @State private var albums: [LibraryAlbum]?
    @EnvironmentObject var spotify: Spotify
    
    public init() {
        self.albums = nil
    }
    
    var body: some View {
        ArtistList()
            .frame(minWidth: 200, maxWidth: .infinity, minHeight: 200, maxHeight: .infinity)
            .onAppear {
                Task {
                    let albums = await spotify.session?.libraryAlbums()
                }
            }
        .toolbar {
            ToolbarItemGroup(placement: .principal) {
                NowPlaying()
                    .frame(minWidth: 500, maxWidth: 500, idealHeight: 32)
                Spacer()
                Button {
                    print("Edit button was tapped")
                } label: {
                    Image(systemName: "shuffle")
                }
            }
            
        }
//        TODO: is there a way to just flat remove the title?
        .navigationTitle("")
    }
}

//struct ContentView_Previews: PreviewProvider {
//    static var previews: some View {
//        ContentView()
//            .previewDevice(PreviewDevice(rawValue: "Mac"))
//    }
//}
