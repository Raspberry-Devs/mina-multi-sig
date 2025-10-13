use mina_hasher::{Fp, Hashable, ROInput};

#[derive(Clone)]
pub struct HashableField(Fp);

impl Hashable for HashableField {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.0)
    }

    fn domain_string(_id: Self::D) -> Option<String> {
        None
    }
}

impl From<Fp> for HashableField {
    fn from(field: Fp) -> Self {
        HashableField(field)
    }
}
