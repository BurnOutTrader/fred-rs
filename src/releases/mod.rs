//! Get all releases of economic data
//! 
//! [https://research.stlouisfed.org/docs/api/fred/releases.html](https://research.stlouisfed.org/docs/api/fred/releases.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::releases::{Builder, SortOrder, OrderBy};
//! use fred_rs::release::Response;
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
//! let mut builder = Builder::new();
//! builder
//!     .limit(5)
//!     .sort_order(SortOrder::Ascending)
//!     .order_by(OrderBy::ReleaseId);
//! 
//! let resp: Response = match c.releases(Some(builder)) {
//! Ok(resp) => resp,
//! Err(msg) => {
//!     println!("{}", msg);
//!     assert_eq!(2, 1);
//!     return
//!     },
//! };
//! 
//! for item in resp.releases {
//!     println!("{}: {}", item.id, item.name);
//! }
//! ```

pub mod dates;

// -----------------------------------------------------------------------------

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases.html#order_by](https://research.stlouisfed.org/docs/api/fred/releases.html#order_by)
pub enum OrderBy {
    /// Default
    ReleaseId,
    Name,
    PressRelease,
    RealtimeStart,
    RealtimeEnd,
}

/// Sort order options for the fred/releases endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/releases.html#sort_order](https://research.stlouisfed.org/docs/api/fred/releases.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order (default)
    Ascending,    
    /// Dates returned in descending order
    Descending,   
}

pub struct Builder {
    option_string: String
}

impl Builder {

    /// Initializes a new releases::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::releases::Builder;
    /// // Create a new builder
    /// let mut builder = Builder::new();
    /// // add arguments to the builder
    /// builder
    ///     .realtime_start("1900-01-01")
    ///     .realtime_end("2000-01-01");
    /// ```
    pub fn new() -> Builder {
        Builder {
            option_string: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub(crate) fn build(self) -> String {
        self.option_string
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/releases.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/releases.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

    /// Adds a limit argument to the builder
    /// 
    /// The limit argument specifies a maximum number of observations to return.
    /// 
    /// # Arguments
    /// * `num_results` - Maximum number of results to return
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases.html#limit](https://research.stlouisfed.org/docs/api/fred/releases.html#limit)
    pub fn limit(&mut self, num_results: usize) -> &mut Builder {
        let num_results = if num_results > 1000 { // max value is 1000
            1000
        } else {
            num_results
        };
        self.option_string += format!("&limit={}", num_results).as_str();
        self
    }

    /// Adds an offset argument to the builder
    /// 
    /// Adding an offset shifts the starting result number.  For example, if limit is 5 and offset is 0 then results 1-5 will be returned, but if offset was 5 then results 6-10 would be returned.
    /// 
    /// # Arguments
    /// * `ofs` - the offset amount
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases.html#offset](https://research.stlouisfed.org/docs/api/fred/releases.html#offset)
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases.html#order_by](https://research.stlouisfed.org/docs/api/fred/releases.html#order_by)
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
            OrderBy::ReleaseId => {
                self.option_string += "&order_by=release_id";
            },
            OrderBy::Name => {
                self.option_string += "&order_by=name";
            },
            OrderBy::PressRelease => {
                self.option_string += "&order_by=press_release";
            },
            OrderBy::RealtimeStart => {
                self.option_string += "&order_by=realtime_start";
            },
            OrderBy::RealtimeEnd => {
                self.option_string += "&order_by=realtime_end";
            },
        };
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/releases.html#sort_order](https://research.stlouisfed.org/docs/api/fred/releases.html#sort_order)
    pub fn sort_order(&mut self, order: SortOrder) -> &mut Builder {
        match order {
            SortOrder::Descending => {
                self.option_string += format!("&sort_order=desc").as_str()
            },
            _ => () // ASC is the default so do nothing
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::Response;
    use crate::client::FredClient;

    #[test]
    fn releases_with_options() {
        let c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut builder = Builder::new();
        builder
            .limit(5)
            .sort_order(SortOrder::Ascending)
            .order_by(OrderBy::ReleaseId);

        let resp: Response = match c.releases(Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.releases {
            println!("{}: {}", item.id, item.name);
        }
    } 
}