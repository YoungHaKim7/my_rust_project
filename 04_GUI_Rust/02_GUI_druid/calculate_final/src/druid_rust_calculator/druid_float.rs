// External crates
use druid::Data;
use rug::Float;

#[derive(Clone)]
//Used in multiple places to manipulate value, so must be public
pub struct DruidFloat {
    pub value: Float,
}

impl Data for DruidFloat {
    fn same(&self, other: &Self) -> bool {
        Float::eq(&self.value, &other.value)
    }
}
