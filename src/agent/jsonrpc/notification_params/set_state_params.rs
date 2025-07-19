use serde::Deserialize;
use serde::Serialize;

use crate::agent_desired_state::AgentDesiredState;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetStateParams {
    pub desired_state: AgentDesiredState,
}
