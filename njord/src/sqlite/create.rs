//! BSD 3-Clause License
//!
//! Copyright (c) 2024, Marcus Cvjeticanin
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions are met:
//!
//! 1. Redistributions of source code must retain the above copyright notice, this
//!    list of conditions and the following disclaimer.
//!
//! 2. Redistributions in binary form must reproduce the above copyright notice,
//!    this list of conditions and the following disclaimer in the documentation
//!    and/or other materials provided with the distribution.
//!
//! 3. Neither the name of the copyright holder nor the names of its
//!    contributors may be used to endorse or promote products derived from
//!    this software without specific prior written permission.
//!
//! THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//! AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//! DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//! SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//! CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//! OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//! OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::sqlite::util::remove_quotes_and_backslashes;

use rusqlite::{Connection, Result};

use log::info;

use crate::table::Table;

/// Constructs a new CREATE TABLE query builder.
///
/// # Arguments
///
/// * `conn` - A `rusqlite::Connection` to the SQLite database.
///
/// # Returns
///
/// A `CreateTableBuilder` instance.
pub fn create_table<T: Table + Default>(conn: Connection, table: T) -> CreateTableBuilder<T> {
    CreateTableBuilder::new(conn, table)
}

/// A builder for constructing CREATE TABLE queries.
pub struct CreateTableBuilder<T: Table + Default> {
    conn: Connection,
    table: T,
}

impl<T: Table + Default> CreateTableBuilder<T> {
    /// Creates a new `CreateTableBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `conn` - A `rusqlite::Connection` to the SQLite database.
    pub fn new(conn: Connection, table: T) -> Self {
        CreateTableBuilder { conn, table }
    }

    /// Builds and executes the DELETE query.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the deletion operation.
    pub fn build(mut self) -> Result<(), String> {
        let table_name = self.table.get_name();
        let mut columns = self
            .table
            .get_columns()
            .iter()
            .map(|c| format!("{} {}", c.0, remove_quotes_and_backslashes(c.1)))
            .collect::<Vec<String>>();

        columns.sort_by(|a, b| b.contains("PRIMARY KEY").cmp(&a.contains("PRIMARY KEY")));

        // Sanitize table name from unwanted quotations or backslashes
        let table_name_str = remove_quotes_and_backslashes(&table_name);
        let columns_str = columns.join(",");

        // Construct the query based on defined variables above
        let query = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" ({});",
            table_name_str, columns_str
        );

        info!("{}", query);
        println!("{}", query);

        // Execute SQL
        let _ = self.conn.execute(&query, []);

        Ok(())
    }
}
