use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineParameters {
    pub max_tokens: i64,
    pub context_window: i64,

    pub temperature: f64,
    pub top_k: i64,
    pub top_p: f64,

    pub typical_p: f64,
    pub min_p: f64,
    pub top_a: f64,

    pub tfs: f64,
    pub epsilon_cutoff: f64,
    pub eta_cutoff: f64,

    pub repetition_penalty: f64,
    pub repetition_penalty_range: i64,
    pub encoder_penalty: f64,

    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub no_repeat_ngram_size: f64,

    pub smoothing_factor: f64,
    pub smoothing_curve: f64,

    pub dry_muiltiplier: f64,
    pub dry_base: f64,
    pub dry_allowed_length: i64,
    pub dry_sequence_breakers: Vec<String>,

    pub dt: bool,
    pub dt_min_temperature: f64,
    pub dt_max_temperature: f64,

    pub mirostat: bool,
    pub mirostat_mode: i64,
    pub mirostat_tau: f64,
    pub mirostat_eta: f64,

    pub bs: bool,
    pub bs_n: i64,
    pub bs_lenth_penalty: f64,
    pub bs_early_stopping: bool,

    pub cs: bool,
    pub cs_penalty_alpha: f64,

    pub do_sample: bool,
    pub add_beos_token: bool,
    pub ban_beos_token: bool,
    pub skip_special_tokens: bool,
    pub temperature_last: bool,

    pub banned_tokens: Vec<i64>,
    pub banned_strings: Vec<String>,

    pub stop_sequences: Vec<String>,
}

impl Default for EngineParameters {
    fn default() -> Self {
        EngineParameters {
            max_tokens: 512,
            context_window: 4096,

            temperature: 0.5,
            top_k: 50,
            top_p: 0.8,

            typical_p: 1.0,
            min_p: 0.1,
            top_a: 0.0,

            tfs: 1.0,
            epsilon_cutoff: 0.0,
            eta_cutoff: 0.0,

            repetition_penalty: 1.05,
            repetition_penalty_range: 1024,
            encoder_penalty: 1.0,

            frequency_penalty: 1.05,
            presence_penalty: 0.0,
            no_repeat_ngram_size: 0.0,

            smoothing_factor: 0.33,
            smoothing_curve: 1.0,

            dry_muiltiplier: 0.8,
            dry_base: 1.75,
            dry_allowed_length: 2,
            dry_sequence_breakers: vec![
                "\n".to_string(),
                ":".to_string(),
                "\"".to_string(),
                "*".to_string(),
            ],

            dt: false,
            dt_min_temperature: 0.0,
            dt_max_temperature: 2.0,

            mirostat: false,
            mirostat_mode: 0,
            mirostat_tau: 5.0,
            mirostat_eta: 0.1,

            bs: false,
            bs_n: 1,
            bs_lenth_penalty: 0.0,
            bs_early_stopping: false,

            cs: false,
            cs_penalty_alpha: 0.0,

            do_sample: true,
            add_beos_token: true,
            ban_beos_token: false,
            skip_special_tokens: false,
            temperature_last: true,

            banned_tokens: vec![],
            banned_strings: vec![],

            stop_sequences: vec![],
        }
    }
}
