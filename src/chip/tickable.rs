use std::collections::VecDeque;

pub trait Tickable {
    fn tick(&mut self) {
        let mut updated_this_tick = vec![false; self.get_num_components()];

        let inputs: Vec<usize> = self.get_input_ids();
        let mut queue: VecDeque<usize> = VecDeque::from(inputs);

        while let Some(index) = queue.pop_front() {
            if updated_this_tick[index] == true {
                continue;
            }

            self.update_node(&index);
            updated_this_tick[index] = true;

            if let Some(targets) = self.get_forward_links_for(&index) {
                queue.extend(targets.iter().copied());
            }
        }
    }

    fn get_num_components(&self) -> usize;
    fn get_input_ids(&self) -> Vec<usize>;
    fn update_node(&mut self, index: &usize);
    fn get_forward_links_for(&mut self, index: &usize) -> Option<&Vec<usize>>;
}
