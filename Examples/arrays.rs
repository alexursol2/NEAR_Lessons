use near_sdk::near;

#[near(contract_state)]
pub struct Contract {

}

#[near]
impl Contract {
    pub fn track_player_scores() -> String {
        // Initialize all players with starting score
        let starting_scores: [u32; 5] = [100; 5]; // All players start with 100 points
        let init_result = format!("Starting scores: {:?}", starting_scores);
        
        // Store scores for 5 players - fixed array size
        let mut player_scores: [u32; 5] = [150, 200, 180, 220, 170];
        let current_player = format!("Player 3 score: {}", player_scores[2]); // Prints: Player 3 score: 180
            
        // Update scores
        player_scores[2] = 250; // Player 3 gets bonus points
        let bonus_result = format!("Player 2 new score: {}", player_scores[2]); // Prints: Player 3 new score: 250
            
        format!("{} | {} | {}", current_player, bonus_result, init_result)
    }

    // Function 2: Manage voting results for proposals
    pub fn manage_voting_results() -> String {
        let mut proposal_votes: [u32; 5] = [42, 15, 96, 23, 87];
            
        // Count total number of proposals
        let total_proposals = proposal_votes.len(); // Returns 5
            
        // Find proposal with highest votes (winning proposal) - modern way
        let winning_votes = proposal_votes.iter().max().copied().unwrap();
            
        // Check if any proposal reached minimum threshold - modern way
        let threshold = 50;
        let passed = proposal_votes.contains(&threshold);
            
        // Swap positions of two proposals (e.g., proposal 0 and proposal 2)
        proposal_votes.swap(0, 2);
        let swap_result = format!("After swap(0,2): {:?}", proposal_votes);
            
        // Sort proposals by vote count for ranking
        proposal_votes.sort();
            
        format!(
            "Total: {} | Max votes: {} | Contains {}: {} | {} | Sorted: {:?}", 
            total_proposals, winning_votes, threshold, passed, swap_result, proposal_votes
        )
    }
}