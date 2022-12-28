use std::ptr;

pub struct Input{
    pub id: String,
    values: Vec<f32>,
    measures: Vec<Measure>,
    start_year: u32,
    end_year: u32,
}

impl Input{

    pub fn new(id: String, start_year: u32, end_year: u32) -> Input{
        return Input{
            id: id,
            values: vec![0.0; (end_year-start_year+1) as usize],
            measures: Vec::from([]),
            start_year: start_year,
            end_year: end_year,
        }
    }

    pub fn get_year(&self, year: u32) -> f32{
        return self.values[(year-self.start_year) as usize];
    }

    pub fn apply_measures(&mut self) {
        for year_index in 1..(self.end_year-self.start_year+1) {
            self.values[year_index as usize] =
                self.values[(year_index - 1) as usize];
            for measure in &self.measures {
                self.values[year_index as usize] +=
                   measure.delta_per_year(year_index + self.start_year);
            }
        }
    }

    pub fn set_values(&mut self, value: f32){
        self.values = vec![value; (self.end_year-self.start_year+1) as usize];
        self.apply_measures();
    }

    pub fn get_value(& self) -> f32{
        return self.values[0];
    }

    pub fn add_measure(&mut self, id: &str, start_year: u32, end_year: u32, delta: f32){
        self.measures.push(Measure{
            id: (self.id.to_owned()+"/"+id).to_string(),
            start_year: start_year,
            end_year: end_year,
            delta: delta,
        });
    }

    fn get_measure_by_id(&mut self, measure_id: &str) -> Option<&mut Measure>{
        for measure in &mut self.measures{
            if measure.id==measure_id.to_owned(){
                return Some(measure);
            }
        }
        return None;
    }

    pub fn update_measure(&mut self, measure_id: &str, start_year: u32, end_year: u32, delta: f32){
        let res = self.get_measure_by_id(measure_id);
        match res{
            Some(measure) => {
                measure.update(start_year, end_year, delta);
            },
            None => (),
        }
    }

    fn get_measure_index_by_id(&mut self, measure_id: &str) -> Option<usize>{
        return self.measures.iter().position(|m| ptr::eq(&m.id, &measure_id.to_owned()));
    }

    pub fn delete_measure(&mut self, measure_id: &str){
        let res = self.get_measure_index_by_id(measure_id);
        match res{
            Some(index) => {
                self.measures.remove(index);
            },
            None => (),
        }
    }

    pub fn list_measure_ids(&self) -> Vec<&String>{
        return self.measures.iter().map(|a| &a.id).collect();
    }

}

pub struct Measure {
    id: String,
    start_year: u32,
    end_year: u32,
    delta: f32,
}

impl Measure{

    pub fn delta_per_year(&self, year: u32) -> f32{
        // start_yaer is the first yaer in which the measure is applied
        // end year is the last year in which the measures is applied
        // the target delta is reached at the end of the end_year
        if year < self.start_year || year > self.end_year{
            return 0.0;
        }
        return self.delta / (self.start_year - self.end_year + 1) as f32
    }

    pub fn update(&mut self, start_year: u32, end_year: u32, delta: f32){
        self.start_year = start_year;
        self.end_year = end_year;
        self.delta = delta;
    }

}

pub trait InputFields{
    fn get_inputs(& self) -> Vec<&Input>;
    fn get_input_by_id(&mut self, id: &str) -> Option<&mut Input>;
}
