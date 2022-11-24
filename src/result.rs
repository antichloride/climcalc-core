
pub struct Results{
    pub id: String,
    values: Vec<f32>,
    start_year: u32,
}

impl Results{

    pub fn new(id: String, start_year: u32, end_year: u32) -> Results{
        return Results{
            id: id,
            values: vec![0.0; (end_year-start_year+1) as usize],
            start_year: start_year,
        }
    }

    pub fn get_year(&self, year: u32) -> f32{
        return self.values[(year-self.start_year) as usize];
    }

    pub fn sum_years(&self, start_year: u32, end_year: u32) -> f32{
        return self.values
            [(start_year-self.start_year) as usize
            .. (end_year-self.start_year+1) as usize]
            .iter().sum();
    }

    pub fn set_year_value(&mut self, year: u32, value: f32){
        let year_index = (year - self.start_year) as usize;
        self.values[year_index] = value;
    }


    pub fn get_values(& self) -> &Vec<f32>{
        return & self.values;
    }
}
