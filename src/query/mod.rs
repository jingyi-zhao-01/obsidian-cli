// Query module for retrieving and searching vault data
mod search;
mod links;
mod tags;

pub use search::{search_chunks, SearchResult};
pub use links::{get_backlinks, get_forward_links, get_unresolved_links, LinkResult};
pub use tags::{list_tags, get_notes_by_tag, get_notes_by_tags_and, get_notes_by_tags_or, TagResult};
