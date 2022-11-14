# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} head
# @return {ListNode}
def detectCycle(head)
    slow = head
    fast = head
    # `slow` and `fast` must meet somewhere if there is a cycle!
    loop do
        slow = slow&.next
        fast = fast&.next&.next
        return nil if slow.nil? || fast.nil?
        break if slow == fast
    end
    # The asnwer is `pos`-away from the `slow`,
    # which is `pos`-away from the `head` too!
    while head != slow
        slow = slow.next
        head = head.next
    end
    head
end
