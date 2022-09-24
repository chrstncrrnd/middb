use std::{fs::File, io::{Read}};
struct DB{
    file_path: String,
}


impl DB{
    fn new(db_file_path: String) -> Self{
        Self { file_path: db_file_path }
    }

    fn load(&self){
        if let Ok(mut file) = File::open(self.file_path.clone()){
            // 1mib buffer (should not rly exceed this lol) if so we have a big problem
            let mut buffer: &mut [u8; 1048576] = &mut [0_u8; 1048576];

            file.read(buffer).unwrap();
            
            


        }else{
            File::create(self.file_path.clone()).unwrap();
        }
    }

    fn save(&self){

    }
}