pub type Uuid = u32;

pub struct SequentialIdGenerator {
    counter: u32
}

/// sequential id generator
/// generates ids that are sequential, guarnteed to be unique
/// for the first 2,147,483,647 ids that are generated 
impl SequentialIdGenerator {
    /// get a sequential id from the generator
    pub fn get_id(&mut self) -> u32 {
        self.counter += 1;
        return self.counter - 1;
    }
}

impl Default for SequentialIdGenerator {
    /// generate new sequential id generator
    fn default() -> SequentialIdGenerator {
        return SequentialIdGenerator { 
            counter:0
        };
    }
}

//TODO create type seqid and implement equals trait
//reserve id 0 as undefined and hvae enum containing value, Unassigned, Id(u32)
//the type will of the type enum