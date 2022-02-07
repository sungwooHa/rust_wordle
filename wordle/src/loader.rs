use ini::Ini;

static LANG_PACK_FILE_NAME : &'static str = "MSG_us.ini";

pub mod loader{
    pub fn load_language() -> Option<_>{

        let language_pack = match Ini::load_from_file(LANG_PACK_FILE_NAME){
            Ok(pack) => pack.unwrap(),
            None => {
                println!("can't load language pack. {} ", LANG_PACK_FILE_NAME);
                return None;
            }
        };

        
    }
}