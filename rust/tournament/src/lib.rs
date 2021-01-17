use std::collections::BTreeMap;
#[derive(Default, Clone)]
struct GameState {
    won: u32,
    lose: u32,
    draw: u32,
    score: u32,
    rounds: u32,
}

#[derive(Clone)]
pub struct GameRecord {
    map: BTreeMap<String, GameState>,
}

pub fn set_record(team: &str, state: &str, rec: &mut GameRecord) {
    //println!("team:{}, state:{}", team, state);
    let mut gs = match rec.map.get(team) {
        Some(t) => t.clone(),
        None => GameState::default(),
    };
    match state {
        "win" => {
            gs.won += 1;
            gs.rounds += 1;
            gs.score += 3;
            rec.map.insert(team.to_string(), gs);
        }
        "loss" => {
            gs.lose += 1;
            gs.rounds += 1;
            rec.map.insert(team.to_string(), gs);
        }
        "draw" => {
            gs.draw += 1;
            gs.rounds += 1;
            gs.score += 1;
            rec.map.insert(team.to_string(), gs);
        }
        _ => {}
    }
}

pub fn print_record_string(records: &GameRecord) -> String {
    let mut index = 0;
    let mut ret = "Team                           | MP |  W |  D |  L |  P".to_string();
    let mut hash_vec: Vec<(&String, &GameState)> = records.map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.score.cmp(&a.1.score));
    for (k, v ) in hash_vec {
        if index != records.map.len() {
            ret.push('\n');
        }
        ret.push_str(&format!(
            "{:30} |  {} |  {} |  {} |  {} |  {}",
            k, v.rounds, v.won, v.draw, v.lose, v.score
        ));
        index += 1;
    }
    ret
}

pub fn tally(match_results: &str) -> String {
    let mut rec = GameRecord {
        map: BTreeMap::new(),
    };
    for line in match_results.to_string().lines() {
        let svec: Vec<&str> = line.split(';').collect();
        let sres = svec[2];
        let tres = match svec[2] {
            "win" => {
                "loss"
            }
            "loss" => {
                "win"
            }
            "draw" => {
                "draw"
            }
            _ => {""}
        };
        set_record(svec[0], sres, &mut rec);
        set_record(svec[1], tres, &mut rec);
    }
    print_record_string(&rec)
}
