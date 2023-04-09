use serde::ser::{self, Serializer};

use serde_rison::ser::to_string;

#[test]
fn test_bool() {
    assert_eq!(to_string(&true).as_str(), "!t");
    assert_eq!(to_string(&false).as_str(), "!f");
}
