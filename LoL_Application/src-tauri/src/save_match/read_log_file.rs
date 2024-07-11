use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn read_log_file(path: &str) -> Result<(Vec<(String, String)>, Vec<(String, String)>), String> {
    println!("리드 파일 실행");

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("파일을 열 수 없습니다: {}", e)),
    };

    let reader = BufReader::new(file);
    let mut team_order = Vec::new();
    let mut team_chaos = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("파일을 읽는 중 오류 발생: {}", e))?;
        if line.contains("ROST| CONNECTION READY") {
            println!("라인에 CONNECTION READY 포함");
            if line.contains("CONNECTION READY | TeamOrder") {
                println!("라인에 TeamOrder 포함");
                if let Some(info) = extract_info(&line) {
                    println!("TeamOrder: {:?}", info);
                    team_order.push(info);
                } else {
                    println!("TeamOrder 정보 추출 실패");
                }
            } else if line.contains("CONNECTION READY | TeamChaos") {
                println!("라인에 TeamChaos 포함");
                if let Some(info) = extract_info(&line) {
                    println!("TeamChaos: {:?}", info);
                    team_chaos.push(info);
                } else {
                    println!("TeamChaos 정보 추출 실패");
                }
            }
        }
    }

    Ok((team_order, team_chaos))
}

fn extract_info(line: &str) -> Option<(String, String)> {
    println!("추출 라인 {}", line);

    let nickname_regex = Regex::new(r"'([^']+)'").unwrap();
    let champion_regex = Regex::new(r"Champion\(([^)]+)\)").unwrap();

    let nickname = nickname_regex.captures(line).and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));
    let champion = champion_regex.captures(line).and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

    if let (Some(nickname), Some(champion)) = (nickname, champion) {
        println!("닉네임: {}, 챔피언: {}", nickname, champion);
        return Some((nickname, champion));
    }

    None
}
