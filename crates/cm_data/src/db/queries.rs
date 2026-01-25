//! SQL query helpers.

/// Query builder helper.
pub struct QueryBuilder {
    query: String,
}

impl QueryBuilder {
    /// Create a new query builder.
    pub fn new(base: &str) -> Self {
        Self {
            query: base.to_string(),
        }
    }

    /// Add WHERE clause.
    pub fn where_clause(mut self, condition: &str) -> Self {
        if self.query.contains("WHERE") {
            self.query.push_str(" AND ");
        } else {
            self.query.push_str(" WHERE ");
        }
        self.query.push_str(condition);
        self
    }

    /// Add ORDER BY.
    pub fn order_by(mut self, column: &str, asc: bool) -> Self {
        self.query.push_str(" ORDER BY ");
        self.query.push_str(column);
        if asc {
            self.query.push_str(" ASC");
        } else {
            self.query.push_str(" DESC");
        }
        self
    }

    /// Add LIMIT.
    pub fn limit(mut self, n: usize) -> Self {
        self.query.push_str(&format!(" LIMIT {}", n));
        self
    }

    /// Build the query string.
    pub fn build(self) -> String {
        self.query
    }
}
