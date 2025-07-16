use near_sdk::near;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub enum VoteChoice{
    Yes,
    No,
    Abstain,
}

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub enum ProposalStatus{
    Active,
    Passed,
    Rejected,
}

#[near(contract_state)]
#[derive(Default)]  // Added Default trait
pub struct VotingContract{
    proposal_title: String,
    yes_votes: u32,
    no_votes: u32,
    abstain_votes: u32,
    voting_deadline: u64,
    min_votes_required: u32,
    is_voting_open: bool
}

#[near]
impl VotingContract{
    #[init]
    pub fn new(title: String, deadline: u64, min_votes: u32) -> Self{
        Self{
            proposal_title: title,
            yes_votes: 0,
            no_votes: 0,
            abstain_votes: 0,
            voting_deadline: deadline,
            min_votes_required: min_votes,
            is_voting_open: true
        }
    }

    pub fn is_valid_vote_time(&self, current_time: u64) -> bool{
        if current_time > self.voting_deadline{
            false
        }
        else{
            true
        }
    }

    // Changed voter_id from &str to String
    pub fn can_vote(&self, current_time: u64, voter_id: String) -> String{
        if !self.is_voting_open{
            "Voting is closed".to_string()
        } else if current_time > self.voting_deadline{
            "Voting deadline has passed".to_string()
        } else if voter_id.is_empty(){
            "Invalid voter ID".to_string()
        } else{
            "You can vote".to_string()
        }
    }

    pub fn process_vote(&mut self, choice: VoteChoice) -> String{
        match choice{
            VoteChoice::Yes => {
                self.yes_votes = self.yes_votes + 1;
                "Yes vote recorded".to_string()
            }
            VoteChoice::No => {
                self.no_votes = self.no_votes + 1;
                "No vote recorded".to_string()
            }
            VoteChoice::Abstain => {
                self.abstain_votes = self.abstain_votes + 1;
                "Abstain vote recorded".to_string()
            }
        }
    }

    pub fn get_participation_level(&self) -> String {
        let total_votes = self.yes_votes + self.no_votes + self.abstain_votes;

        match total_votes{
            0 => "No participation".to_string(),
            1..=10 => "Low participation".to_string(),
            11..100 => "Medium participation".to_string(),
            _ => "High participation".to_string(),
        }
    }

    pub fn matchcondition(&self) -> ProposalStatus{
        let total_votes = self.yes_votes + self.no_votes + self.abstain_votes;

        match(total_votes >= self.min_votes_required, self.yes_votes > self.no_votes){
            (false, _) => ProposalStatus::Active,
            (true, true) => ProposalStatus::Passed,
            (true, false) => ProposalStatus::Rejected,
        }
    }

    pub fn add_yes_votes(&mut self, count:u32) -> String{
        for _n in (0..=count).step_by(2){
            self.yes_votes = self.yes_votes + 1;
        }

        format!("Added {} yes votes", count)
    }

    pub fn count_until_majority(&self) -> u32{
        let mut count = 0;
        let total_votes = self.yes_votes + self.no_votes;
        let mut yescount = 0;

        while yescount <= total_votes / 2 {
            count = count + 1;
            yescount = yescount + 1;
        }

        count
    }

    pub fn find_winning_threshold(&self) -> u32{
        let mut threshold = 1;

        loop {
            if threshold == 10 {
                break;
            }
            else{
                threshold = threshold + 1;
            }
        }

        threshold
    }
}
