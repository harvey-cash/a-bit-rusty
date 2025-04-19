pub struct Link {
    pub source: usize,
    pub target: usize,
}

impl Link {
    pub fn new(source: usize, target: usize) -> Self {
        Link { source, target }
    }
}

pub struct Chip {
    input_target_lookup: Vec<usize>,
    num_outputs: usize,
    num_nands: usize,
    links: Vec<Link>,
    input_values: Vec<u8>,
    output_values: Vec<u8>
}

impl Chip {
    pub fn new(num_inputs: usize, num_outputs: usize, num_nands: usize, links: Vec<Link>) -> Self {
        if num_inputs == 0 || num_outputs == 0 || links.len() == 0 {
            panic!("Chip must have at least one input, one output, and one link!")
        }
        
        let max_node_index: usize = num_inputs + num_outputs + num_nands - 1;

        let link_out_of_range = |link: &Link| -> bool 
            { link.source > max_node_index || link.target > max_node_index };
        
        if links.iter().any(link_out_of_range) {
            panic!("Bad link!")
        }

        let input_target_lookup = Self::get_targets_for_inputs(num_inputs, &links);
        let input_values = vec![0; num_inputs];
        let output_values = vec![0; num_outputs];

        Chip { input_target_lookup, num_outputs, num_nands, links, input_values, output_values }
    }

    pub fn set_input(&mut self, index: usize, value: u8) {
        self.input_values[index] = value;
    }

    pub fn update(&mut self) {
        for (i, target) in self.input_target_lookup.iter().enumerate() {
            let output_index = self.get_output_value_index(target);
            self.output_values[output_index] = self.input_values[i];
        }
    }

    pub fn get_output(&self, index: &usize) -> u8 {
        let output_index = self.get_output_value_index(index);
        self.output_values[output_index]
    }

    fn get_output_value_index(&self, output_index: &usize) -> usize {
        output_index - self.input_target_lookup.len()
    }

    fn get_targets_for_inputs(num_inputs: usize, links: &Vec<Link>) -> Vec<usize> {
        let mut targets = vec![0; num_inputs];
        for link in links {
            if link.source < num_inputs {
                targets[link.source] = link.target;
            }
        }
        targets
    }
}
