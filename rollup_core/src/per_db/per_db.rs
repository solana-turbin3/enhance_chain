use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;

pub struct PERTXS {
    signatueres : Vec<String>
}
pub struct PerDB {
    data : HashMap<Pubkey,PERTXS>
}

pub struct FullPerDB {
    full_per_db : Vec<PerDB>
}
  

// add tsx
// get data of a tsx

impl FullPerDB {

    pub fn add(&mut self,app_program_id:Pubkey, tsx_signature : String) {

          if let Some(per_db) = self.full_per_db.iter_mut()
          .find(|per_db| per_db.data.contains_key(&app_program_id)) {
            if let Some(pertxs) = per_db.data.get_mut(&app_program_id) {
                pertxs.signatueres.push(tsx_signature);
            }
          } else {
            let mut per_db = HashMap::new();
            per_db.insert(
                app_program_id,
                PERTXS {
                    signatueres : vec![tsx_signature]
                }
            );
            self.full_per_db.push(
                PerDB {
                    data : per_db
                }
            );
          }
    }

    pub fn get_signature_for_add(&mut self, app_program_id:Pubkey) -> Option<Vec<String>> {
        if let Some(per_db) = self.full_per_db.iter_mut()
        .find(|per_db| per_db.data.contains_key(&app_program_id)) {
            if let Some(data) = per_db.data.get_mut(&app_program_id) {
                Some(data.signatueres.clone())
            } else {
                None
            }
         } else {
            None
         }
    }
}