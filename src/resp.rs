use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::book::L2Book;

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    L2BookOk(L2Book),
    CancelOk,
    CancelErr,
    PlaceOk(Uuid),
    PlacErr,
}