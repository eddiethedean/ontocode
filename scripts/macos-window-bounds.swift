import CoreGraphics
import Foundation

func isVsCodeOwner(_ name: String) -> Bool {
    let n = name.lowercased()
    return n == "code"
        || n == "electron"
        || n.hasPrefix("code helper")
        || n.contains("visual studio code")
        || n.contains("code - oss")
}

let opts = CGWindowListOption(arrayLiteral: .optionOnScreenOnly, .excludeDesktopElements)
guard let info = CGWindowListCopyWindowInfo(opts, kCGNullWindowID) as? [[String: Any]] else {
    exit(0)
}

var best: (score: Double, line: String)?
for w in info {
    let name = (w[kCGWindowOwnerName as String] as? String) ?? ""
    let title = ((w[kCGWindowName as String] as? String) ?? "").lowercased()
    let layer = w[kCGWindowLayer as String] as? Int ?? 0
    guard layer == 0, isVsCodeOwner(name) else { continue }
    guard let b = w[kCGWindowBounds as String] as? [String: Any] else { continue }
    let x = (b["X"] as? NSNumber)?.doubleValue ?? 0
    let y = (b["Y"] as? NSNumber)?.doubleValue ?? 0
    let width = (b["Width"] as? NSNumber)?.doubleValue ?? 0
    let height = (b["Height"] as? NSNumber)?.doubleValue ?? 0
    guard width >= 600, height >= 400 else { continue }
    guard let number = w[kCGWindowNumber as String] as? Int else { continue }
    var score = width * height
    if title.contains("extension development") { score *= 4 }
    if title.contains("screenshot") || title.contains("ontocode") || title.contains("query")
        || title.contains("reasoner") || title.contains("diff") || title.contains("inspector")
        || title.contains("human") || title.contains("class:")
    {
        score *= 1.5
    }
    let line = "\(number),\(Int(x)),\(Int(y)),\(Int(width)),\(Int(height))"
    if best == nil || score > best!.score { best = (score, line) }
}
if let best {
    print(best.line)
}
