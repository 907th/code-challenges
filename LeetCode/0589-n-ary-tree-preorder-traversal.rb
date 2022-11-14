# Definition for a Node.
# class Node
#     attr_accessor :val, :children
#     def initialize(val)
#         @val = val
#         @children = []
#     end
# end

# @param {Node} root
# @return {List[int]}
def preorder(root, list = [])
    return list if root.nil?
    list.push(root.val);
    root.children.each do |child|
        preorder(child, list)
    end
    list
end
