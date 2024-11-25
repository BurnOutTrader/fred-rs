//! Get a category
//! 
//! [https://research.stlouisfed.org/docs/api/fred/category.html](https://research.stlouisfed.org/docs/api/fred/category.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::category::Response;
//! 
//! let mut c = match FredClient::new() {
//! Ok(c) => c,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! let resp: Response = match c.category(125) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! for s in resp.categories {
//!     println!("ID: {}  Name: {}  ParentID: {}", s.id, s.name, s.parent_id);
//! }
//! ```

pub mod children;
pub mod related;
pub mod series;
pub mod tags;
pub mod related_tags;

// -----------------------------------------------------------------------------

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Clone, Debug, Default)]
/// Response data structure for a collection of categories
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html] (https://research.stlouisfed.org/docs/api/fred/category.html)
pub struct Response {
    /// List of categories returned by the query
    pub categories: Vec<Category>,
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.categories.iter() {
            match item.fmt(f) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            match writeln!(f, "") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
/// Data structure containing infomation about a particular category
/// 
/// [https://research.stlouisfed.org/docs/api/fred/category.html](https://research.stlouisfed.org/docs/api/fred/category.html)
pub struct Category {
    /// The category ID number
    pub id: usize,
    /// The category name
    pub name: String,
    /// The parent ID number of the category
    pub parent_id: usize,
    /// Additional information about the category
    pub notes: Option<String>,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Category {}: {}", self.id, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn category_no_options() {
        let c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: Response = match c.category(125) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for s in resp.categories {
            println!("ID: {}  Name: {}  ParentID: {}", s.id, s.name, s.parent_id);
        }
    } 
}