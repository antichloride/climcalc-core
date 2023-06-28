use crate::energy::Energy;
use crate::sectors::SectorsInputs;
use crate::sectors::SectorsRawValues;

impl SectorsInputs{
    fn set(&mut self, private: f32, industry:f32, schools: f32, public: f32){
        self.private.set_values(private);
        self.industry.set_values(industry);
        self.schools.set_values(schools);
        self.public.set_values(public);
    }
}

pub fn create_energy(start_year: u32, end_year: u32) -> Energy{

    let mut energy = Energy::new(start_year, end_year);
    return energy
}
