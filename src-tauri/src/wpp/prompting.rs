/* https://github.com/SillyTavern/SillyTavern/blob/ba6f7b7a98cf7a5eaf4f0e81da9779a9a668ced4/default/content/presets/instruct/Llama%203%20Instruct.json
{
    "input_sequence": "<|start_header_id|>user<|end_header_id|>\n\n",
    "output_sequence": "<|start_header_id|>assistant<|end_header_id|>\n\n",
    "last_output_sequence": "",
    "system_sequence": "<|start_header_id|>system<|end_header_id|>\n\n",
    "stop_sequence": "<|eot_id|>",
    "wrap": false,
    "macro": true,
    "names_behavior": "always",
    "activation_regex": "",
    "system_sequence_prefix": "",
    "system_sequence_suffix": "",
    "first_output_sequence": "",
    "skip_examples": false,
    "output_suffix": "<|eot_id|>",
    "input_suffix": "<|eot_id|>",
    "system_suffix": "<|eot_id|>",
    "user_alignment_message": "",
    "system_same_as_user": true,
    "last_system_sequence": "",
    "name": "Llama 3 Instruct"
}
*/

pub struct PromptPreset {
    pub input_sequence: String,
    pub output_sequence: String,
    // pub last_output_sequence: String,
    pub system_sequence: String,
    pub stop_sequence: String,
    // pub wrap: bool,
    // pub macro: bool,
    // pub names_behavior: String,
    // pub activation_regex: String,
    // pub system_sequence_prefix: String,
    // pub system_sequence_suffix: String,
    // pub first_output_sequence: String,
    // pub skip_examples: bool,
    pub output_suffix: String,
    pub input_suffix: String,
    pub system_suffix: String,
    // pub user_alignment_message: String,
    // pub system_same_as_user: bool,
    // pub last_system_sequence: String,
    pub name: String,
}

impl PromptPreset {}

pub fn llama_3_instruct() -> PromptPreset {
    PromptPreset {
        input_sequence: "<|start_header_id|>user<|end_header_id|>\n\n".to_string(),
        output_sequence: "<|start_header_id|>assistant<|end_header_id|>\n\n".to_string(),
        // last_output_sequence: "".to_string(),
        system_sequence: "<|start_header_id|>system<|end_header_id|>\n\n".to_string(),
        stop_sequence: "<|eot_id|>".to_string(),
        // wrap: false,
        // macro: true,
        // names_behavior: "always".to_string(),
        // activation_regex: "".to_string(),
        // system_sequence_prefix: "".to_string(),
        // system_sequence_suffix: "".to_string(),
        // first_output_sequence: "".to_string(),
        // skip_examples: false,
        output_suffix: "<|eot_id|>".to_string(),
        input_suffix: "<|eot_id|>".to_string(),
        system_suffix: "<|eot_id|>".to_string(),
        // user_alignment_message: "".to_string(),
        // system_same_as_user: true,
        // last_system_sequence: "".to_string(),
        name: "Llama 3 Instruct".to_string(),
    }
}
