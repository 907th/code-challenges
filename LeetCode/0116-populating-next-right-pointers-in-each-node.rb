# Definition for Node.
# class Node
#     attr_accessor :val, :left, :right, :next
#     def initialize(val)
#         @val = val
#         @left, @right, @next = nil, nil, nil
#     end
# end

# @param {Node} root
# @return {Node}
def connect(root)
    return root if root.nil?
    connect(root.left)
    connect(root.right)
    l, r = root.left, root.right
    while l && r do
        l.next = r
        l = l.right
        r = r.left
    end
    root
end
