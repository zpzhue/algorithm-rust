/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Left *Node
 *     Right *Node
 *     Next *Node
 * }
 */

func traverse(left, right  *Node) {
	if left == nil || right == nil {
		return
	}
	left.Next = right
	traverse(left.Left, left.Right)
	traverse(left.Right, right.Left)
	traverse(right.Left, right.Right)
}


func connect(root *Node) *Node {
	if root == nil {
		return root
	}
	traverse(root.Left, root.Right)
	return root
}