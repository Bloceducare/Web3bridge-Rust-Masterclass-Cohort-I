use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum CandidateStatus {
    ACCEPTED,
    DECLINED,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Candidates {
    pub id: u32,
    pub name: String,
    pub position: String,
    pub contact: String,
    pub status: CandidateStatus,
}

pub struct CandidatesInfo {
    info: Vec<Candidates>,
    next_id: u32,
    backup: HashMap<u32, Candidates>,
}

impl CandidatesInfo {
    pub fn new() -> Self {
        Self {
            info: Vec::new(),
            next_id: 1,
            backup: HashMap::new(),
        }
    }

    pub fn add_candidate(&mut self, name: String, contact: String, position: String, status: CandidateStatus) -> u32 {
        let present_id = self.next_id;
        let candidate = Candidates {
            id: present_id,
            name,
            position,
            contact,
            status: status,
        };
        self.next_id += 1;
        self.info.push(candidate);
        present_id
    }

    pub fn get_all_candidates(&self) -> &Vec<Candidates> {
        &self.info
    }

    pub fn remove_candidates(&mut self, id: u32) {
        self.info.retain(|candidate_id| candidate_id.id != id)
    }

    pub fn edit_candidates(&mut self, id: u32, new_name: String, new_position: String, new_contact: String) -> bool {
        if let Some(candidate) = self.info.iter_mut().find(|candidate_id| candidate_id.id == id) {
            if !self.backup.contains_key(&id) {
                self.backup.insert(id, candidate.clone());
            }
            candidate.name = new_name;
            candidate.position = new_position;
            candidate.contact = new_contact;
            true
        } else {
            false
        }
    }

    pub fn cancel_edit(&mut self, id: u32) -> bool {
        if let Some(original) = self.backup.remove(&id) {
            if let Some(candidate) = self.info.iter_mut().find(|candidate_id| candidate_id.id == id) {
                *candidate = original;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_candidate() {
        let mut system = CandidatesInfo::new();
        let id = system.add_candidate(
            "John Doe".to_string(),
            "080287653452".to_string(),
            "Developer".to_string(),
            CandidateStatus::ACCEPTED,
        );
        assert_eq!(id, 1);
        assert_eq!(system.get_all_candidates().len(), 1);
    }

    #[test]
    fn test_remove_candidate() {
        let mut system = CandidatesInfo::new();
        let id = system.add_candidate(
            "Jane Smith".to_string(),
            "9289304097368".to_string(),
            "Designer".to_string(),
            CandidateStatus::ACCEPTED,
        );
        system.remove_candidates(id);
        assert_eq!(system.get_all_candidates().len(), 0);
    }

    #[test]
    fn test_edit_candidate() {
        let mut system = CandidatesInfo::new();
        let id = system.add_candidate(
            "Bob Wilson".to_string(),
            "123749787473".to_string(),
            "Manager".to_string(),
            CandidateStatus::ACCEPTED,
        );
        
        let result = system.edit_candidates(id, "Robert Wilson".to_string(), "Senior Manager".to_string(), "93980907923827".to_string());
        assert!(result);
        
        let candidate = &system.get_all_candidates()[0];
        assert_eq!(candidate.name, "Robert Wilson");
        assert_eq!(candidate.position, "Senior Manager");
        assert_eq!(candidate.contact, "93980907923827");
    }

    #[test]
    fn test_cancel_edit() {
        let mut system = CandidatesInfo::new();
        let id = system.add_candidate(
            "Alice Brown".to_string(),
            "0803126823797931".to_string(),
            "Analyst".to_string(),
            CandidateStatus::ACCEPTED,
        );
        
        system.edit_candidates(id, "Alice Johnson".to_string(), "Senior Analyst".to_string(), "0803126823797931".to_string());
        let result = system.cancel_edit(id);
        assert!(result);
        
        let candidate = &system.get_all_candidates()[0];
        assert_eq!(candidate.name, "Alice Brown");
        assert_eq!(candidate.position, "Analyst");
        assert_eq!(candidate.contact, "0803126823797931");
    }

    #[test]
    fn test_edit_nonexistent_candidate() {
        let mut system = CandidatesInfo::new();
        let result = system.edit_candidates(999, "Test".to_string(), "Test".to_string(), "0803122837683631".to_string());
        assert!(!result);
    }

    #[test]
    fn test_cancel_edit_without_backup() {
        let mut system = CandidatesInfo::new();
        let result = system.cancel_edit(999);
        assert!(!result);
    }
}