import UIKit

//
// Lesson 8. Structs (part 1)
//

struct Sport {
    var name: String
    var isOlympic: Bool
    
    var olympicStatus: String {
        if isOlympic {
            return "\(name) is an Olympic sport"
        } else {
            return "\(name) is not an Olympic sport"
        }
    }
}

var tennis = Sport(name: "Tennis", isOlympic: true)
print(tennis.name)
print(tennis.olympicStatus)

struct Task {
    var name: String
    var progress: Int {
        didSet {
            printProgress()
        }
    }
    
    func printProgress() {
        print("Task \(name) is now \(progress)% complete")
    }
    
    mutating func finish() {
        progress = 100
    }
}

var task: Task = Task(name: "Loading data", progress: 0)
task.progress = 10
task.finish()

//
// Props and methods of String and Array
//

var str = "Do or do not, there is no try."

print(str.count)
print(str.hasPrefix("Do"))
print(str.uppercased())
print(str.sorted())

var ary = ["Fizz"]

ary.append("Buzz")
print(ary.count)
print(ary.firstIndex(of: "Buzz"))
ary.remove(at: 0)
print(ary.first)
