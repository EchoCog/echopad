use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelParameters {
    pub batch_n_tokens: usize,
    pub context_size: u32,
    /// The minimum probability for a token to be considered, relative to the probability of the most likely token
    pub min_p: f32,
    pub penalty_frequency: f32,
    /// How many tokens to scan for repetitions (-1 = context size, 0 = disabled)
    pub penalty_last_n: i32,
    pub penalty_presence: f32,
    /// Penalty for repeating tokens (1.0 = disabled)
    pub penalty_repeat: f32,
    /// Adjust the randomness of the generated text (0.0 = greedy/deterministic)
    pub temperature: f32,
    /// Limit the next token selection to the K most probable tokens
    pub top_k: i32,
    /// Limit the next token selection to a subset of tokens with a cumulative probability above a threshold P
    pub top_p: f32,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            batch_n_tokens: 512,
            context_size: 4096,
            min_p: 0.05,
            penalty_frequency: 0.0,
            penalty_last_n: -1,
            penalty_presence: 1.5,
            penalty_repeat: 1.0,
            temperature: 0.6,
            top_k: 40,
            top_p: 0.3,
        }
    }
}
