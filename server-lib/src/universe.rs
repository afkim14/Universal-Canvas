use json_convertible::*;

pub trait Universe<A>: AsJson {
    fn atom_name_singular(&self) -> &str; // "pixel"
    // fn atom_name_plural(&self) -> &str; // "pixels"

    fn update_atom(&mut self, atom: A);

    // fn atom_with_id(&self, id: usize) -> &A;
    // fn atom_with_id_mut(&mut self, id: usize) -> &mut A;
}

pub trait Atom: AsJson + FromJson {
    // fn id(&self) -> &usize;
}
