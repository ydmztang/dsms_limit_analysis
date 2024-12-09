pub enum OrderByOptions {
    Trending,
    Likes,
    Downloads,
}

impl OrderByOptions {
    pub fn as_string(&self) -> &str {
        

        (match self {
            OrderByOptions::Trending => "trending_score",
            OrderByOptions::Likes => "likes",
            OrderByOptions::Downloads => "downloads",
        }) as _
    }
}