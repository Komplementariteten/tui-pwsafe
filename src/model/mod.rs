use crate::contracts::TuiPwSafeErrors;
use rs_pwsafe::pwsdb::record::DbRecord;
use rs_pwsafe::PwFile;
use std::collections::HashSet;
use arboard::Clipboard;

pub struct SafeModel {
    pf: PwFile,
    cl: Clipboard,
}

impl SafeModel {
    pub fn new(path: &str) -> Self {
        let safe = match PwFile::open(path) {
            Ok(f) => f,
            Err(e) => panic!("{}", TuiPwSafeErrors::from(e)),
        };
        let clipboard = Clipboard::new().expect("Can't create clipboard");
        SafeModel { pf: safe, cl:clipboard }
    }

    pub fn to_clipboard(&mut self, content: &str) {
        self.cl.set_text(content.to_string()).expect("can't copy to clipboard");
    }
    
    pub fn unlock(&mut self, phrase: &str) -> Result<(), TuiPwSafeErrors> {
        match self.pf.unlock(phrase) {
            Err(e) => Err(TuiPwSafeErrors::from(e)),
            Ok(_) => Ok(()),
        }
    }

    pub fn by_group_name(&self, name: &str) -> Vec<&DbRecord> {
        self.pf.by_broup(name.to_string())
    }

    pub fn groups(&self) -> HashSet<String> {
        self.pf.groups()
    }
}
