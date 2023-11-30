import readInput from "../Utils/input";
import { NodeType, TreeNode } from "./TreeNode";

const input = readInput('input.txt')

const root = new TreeNode(NodeType.Directory, undefined, new Array<TreeNode>(), 0, '/');

let current_node: TreeNode | undefined = root;

for (const full_cmd of input.slice(1)) {
    const cmd_tree = full_cmd.split(' ');
    const is_command = full_cmd.startsWith('$');

    if (is_command === false) {
        let node_type: NodeType;
        let size: number = 0;

        const prefix = cmd_tree[0];
        const name = cmd_tree[1]

        if (prefix === 'dir') {
            node_type = NodeType.Directory;
            size = 0;
        } else if (prefix.match(/^\d+/g)) { // starts with numbers (is a file)
            node_type = NodeType.File;
            size = Number(prefix);
        }

        current_node?.addChildren(NodeType.File, name, size);
    }
    else {
        const cmd = cmd_tree[1];
        const arg = cmd_tree[2];

        if (cmd === 'cd') {
            if (arg === '..') {
                current_node = current_node?.parent;
            } else {
                current_node = current_node?.getChild(NodeType.Directory, arg);
            }
        }


    }
}


const directory_size_array = new Map<string, number>();

const visited: Array<TreeNode> = [];
let queue = [root];

const SIZE_LIMIT = 70_000_000;
const UPDATE_SPACE_NEEDED: number = 30000000;

while (queue.length > 0) {
    const item = queue.pop();

    if (item && !visited.includes(item)) {
        visited.push(item);

        if (item.type === NodeType.Directory) {
            const s = directory_size_array.get(item.name)
            let itemSize = s ?? item.size;

            if (s) {
                itemSize += item.size;
            }

            directory_size_array.set(item.name, itemSize);
        }

        queue = queue.concat(item.children);

    }
}

const free_space = SIZE_LIMIT - root.size;
const deletion_size = Math.abs(free_space - UPDATE_SPACE_NEEDED);
const dir_size = Array.from(directory_size_array.values()).filter(d => d >= deletion_size).sort((a, b) => a - b).at(0);

console.log("Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory? ", dir_size);
