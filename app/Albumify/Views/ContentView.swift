//
//  ContentView.swift
//  Albumify
//
//  Created by Oliver Cooper on 10/11/22.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        List {
            Text("Book List")
        }
        .toolbar {
            ToolbarItem(placement: .principal) {
                NowPlaying()
                    .frame(minWidth: 500, maxWidth: 500, idealHeight: 32)
            }
        }
        .navigationTitle("")
        
//        .presentedWindowToolbarStyle(.unified(showsTitle: false))
    }
    
    private func recordProgress() {}
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
            .previewDevice(PreviewDevice(rawValue: "Mac"))
    }
}
