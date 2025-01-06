// Copyright (C) 2024 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::env::var;

use once_cell::sync::Lazy;

pub(crate) const CONFIGURATION_TEMPLATE: &str = r#"
version: 0.8
node_id: lambda-searcher
metastore_uri: ${QW_LAMBDA_METASTORE_URI}
default_index_root_uri: s3://${QW_LAMBDA_INDEX_BUCKET}/${QW_LAMBDA_INDEX_PREFIX:-index}
data_dir: /tmp
searcher:
  partial_request_cache_capacity: ${QW_LAMBDA_PARTIAL_REQUEST_CACHE_CAPACITY:-64M}
  fast_field_cache_capacity: 200G
  split_footer_cache_capacity: 1G
  max_num_concurrent_split_searches: 70000
  max_num_concurrent_split_streams: 70000
  aggregation_memory_limit: 5G
"#;

pub(crate) static INDEX_ID: Lazy<String> =
    Lazy::new(|| var("QW_LAMBDA_INDEX_ID").expect("QW_LAMBDA_INDEX_ID must be set"));

pub(crate) static DISABLE_SEARCH_CACHE: Lazy<bool> =
    Lazy::new(|| var("QW_LAMBDA_DISABLE_SEARCH_CACHE").is_ok_and(|v| v.as_str() == "true"));
