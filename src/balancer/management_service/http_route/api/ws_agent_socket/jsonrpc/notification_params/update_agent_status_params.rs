use serde::Deserialize;
use serde::Serialize;

use crate::slot_aggregated_status_snapshot::SlotAggregatedStatusSnapshot;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdateAgentStatusParams {
    pub slot_aggregated_status_snapshot: SlotAggregatedStatusSnapshot,
}
