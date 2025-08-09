use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum CampaignStatus {
    Inactive,
    Active,
    Completed,
}

#[derive(Debug)]
pub struct Campaign {
    pub name: String,
    pub budget: f64,
    pub start_date: String,
    pub end_date: String,
    pub status: CampaignStatus,
    pub can_edit: bool,
}

pub struct AllCampaigns {
    campaigns: HashMap<String, Campaign>,
}

impl AllCampaigns {
    pub fn new() -> Self {
        Self {
            campaigns: HashMap::new(),
        }
    }

    pub fn add_campaign(
        &mut self,
        name: String,
        budget: f64,
        start_date: String,
        end_date: String,
    ) {
        let campaign = Campaign {
            name: name.clone(),
            budget,
            start_date,
            end_date,
            status: CampaignStatus::Inactive,
            can_edit: true,
        };
        self.campaigns.entry(name).or_insert(campaign);
    }

    pub fn update_campaign_status(
        &mut self,
        name: &str,
        status: CampaignStatus,
    ) -> Result<(), &str> {
        if let Some(campaign) = self.campaigns.get_mut(name) {
            if !campaign.can_edit {
                return Err("Campaign cannot be edited");
            }
            campaign.status = status;
            Ok(())
        } else {
            Err("Campaign not found")
        }
    }

    pub fn get_campaign(&self, name: &str) -> Option<&Campaign> {
        self.campaigns.get(name)
    }

    pub fn get_all_campaign(&self) -> Vec<&Campaign> {
        self.campaigns.values().collect()
    }

    pub fn remove_completed_campaign(&mut self, name: &str) -> Result<(), &str> {
        match self.campaigns.get(name) {
            Some(campaign) => {
                if campaign.status == CampaignStatus::Completed {
                    self.campaigns.remove(name);
                    Ok(())
                } else {
                    Err("Campaign is not completed")
                }
            }
            None => Err("Campaign not found"),
        }
    }

    pub fn update_campaign_budget(&mut self, name: &str, new_budget: f64) -> Result<(), &str> {
        match self.campaigns.get_mut(name) {
            Some(campaign) => {
                if !campaign.can_edit {
                    return Err("Campaign cannot be edited");
                }
                campaign.budget = new_budget;
                Ok(())
            }
            None => Err("Campaign not found"),
        }
    }

    pub fn cancel_edit_campaign(&mut self, name: &str) -> Result<(), &str> {
        match self.campaigns.get_mut(name) {
            Some(campaign) => {
                campaign.can_edit = false;
                Ok(())
            }
            None => Err("Campaign not found"),
        }
    }

    pub fn edit_campaign(
        &mut self,
        name: &str,
        new_name: Option<String>,
        new_budget: Option<f64>,
        new_start_date: Option<String>,
        new_end_date: Option<String>,
    ) -> Result<(), &str> {
        if let Some(new_name_value) = new_name {
            if let Some(mut campaign) = self.campaigns.remove(name) {
                if !campaign.can_edit {
                    self.campaigns.insert(name.to_string(), campaign);
                    return Err("Campaign cannot be edited");
                }
                let new_name_key = new_name_value.clone();
                campaign.name = new_name_value.clone();

                if let Some(new_budget) = new_budget {
                    campaign.budget = new_budget;
                }
                if let Some(new_start_date) = new_start_date {
                    campaign.start_date = new_start_date;
                }
                if let Some(new_end_date) = new_end_date {
                    campaign.end_date = new_end_date;
                }

                self.campaigns.insert(new_name_key, campaign);
                Ok(())
            } else {
                Err("Campaign not found")
            }
        } else {
            if let Some(campaign) = self.campaigns.get_mut(name) {
                if !campaign.can_edit {
                    return Err("Campaign cannot be edited");
                }

                if let Some(new_budget) = new_budget {
                    campaign.budget = new_budget;
                }
                if let Some(new_start_date) = new_start_date {
                    campaign.start_date = new_start_date;
                }
                if let Some(new_end_date) = new_end_date {
                    campaign.end_date = new_end_date;
                }
                Ok(())
            } else {
                Err("Campaign not found")
            }
        }
    }
}
