// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues::DataValue;
use common_sql::optimizer::Histogram;
use common_sql::optimizer::HistogramBucket;

#[test]
fn test_histogram() {
    let buckets = vec![
        HistogramBucket::new(DataValue::UInt64(1), 2.0, 1.0),
        HistogramBucket::new(DataValue::UInt64(2), 2.0, 1.0),
    ];

    let histogram = Histogram::new(buckets);
    assert_eq!(histogram.num_buckets(), 2);
    assert_eq!(histogram.num_values(), 4.0);
    assert_eq!(histogram.num_distinct_values(), 2.0);
}