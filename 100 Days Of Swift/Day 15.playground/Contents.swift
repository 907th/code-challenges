import UIKit

//
// Day 15. Swift Review (part 3)
//

struct Person {
    static var belovedColor: String = "red"
    fileprivate var name: String?
    var clothes: String {
        willSet {
            print("I'm changing from \(clothes) to \(newValue)")
        }
        didSet {
            print("I just changed from \(oldValue) to \(clothes)")
        }
    }
    var shoes: String
    var age: Int {
        if shoes == "sneakers" {
            return 17
        } else {
            return 34
        }
    }
    func describe() {
        print("I like wearing \(Person.belovedColor) \(clothes) with \(shoes)")
    }
}
var taylor = Person(clothes: "t-shirts", shoes: "sneakers")
var other = Person(clothes: "shirts", shoes: "high heels")
taylor.describe()
other.describe()
taylor.clothes = "short skirts"
print(other.age)


// Plymorphism
class Album {
    var name: String
    func info() {
        print("Album \(name) is very good!")
    }
    init(name: String) {
        self.name = name
    }
}
class LiveAlbum: Album {
    var place: String
    override func info() {
        print("LiveAlbum \(name) from \(place) is very good!")
    }
    init(name: String, place: String) {
        self.place = place
        super.init(name: name)
    }
}
class StudioAlbum: Album {
    var studio: String
    override func info() {
        print("StudioAlbum \(name) from \(studio) is very good!")
    }
    init(name: String, studio: String) {
        self.studio = studio
        super.init(name: name)
    }
}
let albums: [Album] = [
    LiveAlbum(name: "1", place: "London"),
    StudioAlbum(name: "2", studio: "Warner")
]
for a in albums {
    a.info()
}
if let liveA = albums[0] as? LiveAlbum {
    print(liveA.place)
}
let studioA = albums[1] as! StudioAlbum
print(studioA.studio)
