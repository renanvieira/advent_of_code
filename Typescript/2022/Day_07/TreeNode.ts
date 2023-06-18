export enum NodeType {
    Directory,
    File,
}

export class TreeNode {
    children: Array<TreeNode> = new Array<TreeNode>();
    parent: TreeNode | undefined;

    type: NodeType;
    size: number;
    name: string;

    constructor(type: NodeType, parent: TreeNode | undefined, children: Array<TreeNode>, size: number, name: string) {
        this.type = type;
        this.children = children;
        this.size = size;
        this.parent = parent;
        this.name = name;
    }

    public addChildren(type: NodeType, name: string, size: number = 0) {
        const newChild = new TreeNode(type, this, new Array<TreeNode>(), size, name)
        this.children.push(newChild);

        if (type == NodeType.File) {
            let parent: TreeNode | undefined = this;
            while (parent) {
                parent.size += size;
                parent = parent.parent
            }
        }

        return newChild;
    }

    public getChild(type: NodeType, name: string): TreeNode | undefined {

        for (const node of this.children) {

            if (node.type == type && node.name == name) {
                return node;
            }
        }

        return undefined;
    }
}
