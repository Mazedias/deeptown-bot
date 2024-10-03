use reqwest;
use serde_json::Value;

use crate::util::web_errors::WebRequestError;


/// Calculates the amount of inactive players in the last/current event of a guild
/// Active players are players that that donated in the last/current event.
/// 
/// # Arguments
/// 
/// * `guild_id` - ID of the guild at DeepTown Admin Tools
/// 
/// # Return
/// 
/// On success returns the amount of inactive players else a `util::web_errors::WebRequestError`
pub fn calculate_inactive_players(guild_id: i32) -> Result<i32, WebRequestError> {
    let http_response = request_guild_data(guild_id)
        .map_err(|e| WebRequestError {
            code: 100,
            message: e.to_string(),
        })?;

        
    let players = http_response["players"]["data"]
        .as_array()
        .ok_or_else(|| WebRequestError {
            code: 100,
            message: "Players data not found or in unexpected format".to_string(),
        })?;


    let inactive_counter = players
        .iter()
        .filter(|player| {
            player.get(13)
                .and_then(Value::as_u64)
                .map(|donations| donations == 0)
                .unwrap_or(false)
        })
        .count();
    

    Ok(inactive_counter as i32)
}


/// Calculates the amount of active players in the last/current event of a guild.
/// Active players are players that that donated in the last/current event.
/// 
/// # Arguments
/// 
/// * `guild_id` - ID of the guild at DeepTown Admin Tools
/// 
/// # Return
/// 
/// On success returns the amount of active players else a `util::web_errors::WebRequestError`
pub fn calulate_active_players(guild_id: i32) -> Result<i32, WebRequestError> {
    let http_response = request_guild_data(guild_id)
        .map_err(|e| WebRequestError {
            code: 100,
            message: e.to_string(),
        })?;

    let players = http_response["players"]["data"]
        .as_array()
        .ok_or_else(|| WebRequestError {
            code: 100,
            message: "Players data not found or in unexpected format".to_string(),
        })?;
    
    let active_counter = players
        .iter()
        .filter(|player| {
            player.get(13)
                .and_then(Value::as_u64)  
                .map(|donations| donations != 0)
                .unwrap_or(false)
        })
        .count();

    Ok(active_counter as i32)
}



/// Preforms a HTTP request to dtat.hampl.space for information about 
/// the guild with passed `guild_id`
/// 
/// # Arguments
/// 
/// * `guild_data` - ID of the guild at DeepTown Admin Tools
/// 
/// # Return
/// 
/// On success returns a JSON Object that contains infromation about the guild
/// and all players in that guild
fn request_guild_data(guild_id: i32) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = match reqwest::blocking::get(format!("http://dtat.hampl.space/data/guild/{}", guild_id)) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };

    let json: Value = serde_json::from_str(&resp)?;

    Ok(json)
}