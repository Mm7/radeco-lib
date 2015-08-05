use std::collections::VecDeque;
use petgraph::graph::NodeIndex;
use middle::ssa::{NodeData, SSAMod};

pub fn collect<T: SSAMod<ValueRef=NodeIndex, ActionRef=NodeIndex>>(ssa: &mut T, roots: &[T::ValueRef]) {
	let maxindex = ssa.node_count();
	let mut reachable = Vec::with_capacity(maxindex);
	let mut queue: VecDeque<NodeIndex> = VecDeque::new();

	for i in 0..maxindex {
		reachable.push(match ssa.get_node_data(&NodeIndex::new(i)) {
			NodeData::Op(_, _)      => false,
			NodeData::Comment(_)    => true,
			NodeData::Const(_)      => false,
			NodeData::Phi(_)        => false,
			NodeData::Undefined     => false,
			NodeData::Removed       => true,
			NodeData::BasicBlock(_) => true,
			NodeData::RegisterState => true,
		});
	}

	for root in roots {
		reachable[root.index()] = false;
	}

	queue.extend(roots);

	while let Some(ni) = queue.pop_front() {
		let i = ni.index();

		if reachable[i] {
			continue;
		}

		reachable[i] = true;
		queue.extend(ssa.args_of(ni));
	}

	for i in 0..reachable.len() {
		if !reachable[i] {
			ssa.remove(NodeIndex::new(i));
		}
	}
}