//! Get the related tags for a release
//! 
//! [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html)
//! 
//! ```
//! use fred_rs::client::FredClient;
//! use fred_rs::release::related_tags::{Builder, OrderBy, SortOrder};
//! use fred_rs::tags::Response;
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
//!     .tag_name("usa")
//!     .limit(5)
//!     .sort_order(SortOrder::Descending)
//!     .order_by(OrderBy::Created);
//! 
//! let resp: Response = match c.release_related_tags(9, builder) {
//!     Ok(resp) => resp,
//!     Err(msg) => {
//!         println!("{}", msg);
//!         assert_eq!(2, 1);
//!         return
//!     },
//! };
//! 
//! for item in resp.tags {
//!     println!("{}: {}", item.name, item.created);
//! }
//! ```

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#order_by](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#order_by)
pub enum OrderBy {
    /// Default
    SeriesCount,
    Popularity,
    Created,
    Name,
    GroupId,
}

/// Sort order options for the fred/release/related_tags endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#sort_order](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order (default)
    Ascending,    
    /// Dates returned in descending order
    Descending,   
}

/// A tag group id to filter tags by type.
/// 
/// https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#tag_group_id](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#tag_group_id)
pub enum TagGroupId {
    Frequency,
    General,
    Geography,
    GeographyType,
    Release,
    SeasonalAdjustment,
    Source,
}

pub struct Builder {
    option_string: String,
    tag_names: String,
    exclude_tags: String,
}

impl Builder {

    /// Initializes a new release::related_tags::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::release::related_tags::Builder;
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
            tag_names: String::new(),
            exclude_tags: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub(crate) fn build(mut self) -> Result<String, String> {
        if self.tag_names.len() > 0 {
            self.option_string += format!("&tag_names={}", self.tag_names).as_str()
        }
        if self.exclude_tags.len() > 0 {
            self.option_string += format!("&exclude_tag_names={}", self.exclude_tags).as_str()
        }
        Ok(self.option_string)
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

    /// Adds a tag name to include in the search
    /// 
    /// Results must match all included tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#tag_names](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#tag_names)
    pub fn tag_name(&mut self, tag: &str) -> &mut Builder {
        if self.tag_names.len() != 0 {
            self.tag_names.push(';');
        } 
        self.tag_names += tag;
        self
    }

    /// Adds a tag name to exclude in the search
    /// 
    /// Results must match no excluded tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#exclude_tag_names](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#exclude_tag_names)
    pub fn exclude_tag(&mut self, tag: &str) -> &mut Builder {
        if self.exclude_tags.len() != 0 {
            self.exclude_tags.push(';');
        } 
        self.exclude_tags += tag;
        self
    }

    /// Adds a group id filter to the results
    /// 
    /// # Arguments
    /// * `id` - type by which to filter results
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#tag_group_id](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#tag_group_id)
    pub fn tag_group_id(&mut self, id: TagGroupId) -> &mut Builder {
        match id {
            TagGroupId::Frequency => {
                self.option_string += "&tag_group_id=freq";
            },
            TagGroupId::General => {
                self.option_string += "&tag_group_id=gen";
            },
            TagGroupId::Geography => {
                self.option_string += "&tag_group_id=geo";
            },
            TagGroupId::GeographyType => {
                self.option_string += "&tag_group_id=geot";
            },
            TagGroupId::Release => {
                self.option_string += "&tag_group_id=rls";
            },
            TagGroupId::SeasonalAdjustment => {
                self.option_string += "&tag_group_id=seas";
            },
            TagGroupId::Source => {
                self.option_string += "&tag_group_id=src";
            },
        };
        self
    }

    /// Add search string to find matching tags with
    /// 
    /// # Arguments
    /// * `search_string` - tag name to add
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#search_text](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#search_text)
    pub fn search_text(&mut self, search_string: &str) -> &mut Builder {
        let search_string = search_string.replace(" ", "%20"); // encode for URL
        self.option_string += format!("&tag_search_text={}", search_string).as_str();
        self
    }

    /// Adds a limit argument to the builder
    /// 
    /// The limit argument specifies a maximum number of observations to return.
    /// 
    /// # Arguments
    /// * `num_results` - Maximum number of results to return
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#limit](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#limit)
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
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#offset](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#offset)
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#order_by](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#order_by)
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
            OrderBy::SeriesCount => {
                self.option_string += "&order_by=series_count";
            },
            OrderBy::Popularity => {
                self.option_string += "&order_by=popularity";
            },
            OrderBy::Created => {
                self.option_string += "&order_by=created";
            },
            OrderBy::Name => {
                self.option_string += "&order_by=name";
            },
            OrderBy::GroupId => {
                self.option_string += "&order_by=group_id";
            },
        };
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#sort_order](https://research.stlouisfed.org/docs/api/fred/release_related_tags.html#sort_order)
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
    use crate::tags::Response;
    use crate::client::FredClient;

    #[test]
    fn release_related_tags_with_options_passing() {
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
            .tag_name("usa")
            .limit(5)
            .sort_order(SortOrder::Descending)
            .order_by(OrderBy::Created);

        let resp: Response = match c.release_related_tags(86, builder) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.tags {
            println!(
                "{}: {}",
                item.name,
                item.created,
            );
        }
    }
    
    #[test]
    fn release_related_tags_with_options_failing() {
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
            .sort_order(SortOrder::Descending)
            .order_by(OrderBy::Created);

        let resp: Response = match c.release_related_tags(86, builder) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(1, 1);
                return
            },
        };

        for item in resp.tags {
            println!(
                "{}: {}",
                item.name,
                item.created,
            );
        }

        assert_eq!(2, 1); // if the request succeeded then failure
    }
}