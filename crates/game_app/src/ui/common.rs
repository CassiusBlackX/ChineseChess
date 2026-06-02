use game_view::{AiDifficulty, PlayMode, SessionDto, SnapshotDto};

pub fn format_status(snapshot: &SnapshotDto, game_title: &str) -> String {
    let turn_text = if snapshot.turn > 0 {
        if game_title == "五子棋" {
            "黑方"
        } else {
            "红方"
        }
    } else if game_title == "五子棋" {
        "白方"
    } else {
        "黑方"
    };

    let mut status = format!("{} | 当前回合: {}", snapshot.message, turn_text);

    if let Some(session) = &snapshot.session {
        status.push_str(" | ");
        status.push_str(&format_session(session));
    }

    if snapshot.game_over {
        if snapshot.winner == 0 {
            status.push_str(" | 对局结束: 和棋");
        } else {
            let winner = if snapshot.winner > 0 {
                if game_title == "五子棋" {
                    "黑方"
                } else {
                    "红方"
                }
            } else if game_title == "五子棋" {
                "白方"
            } else {
                "黑方"
            };
            status.push_str(&format!(" | 对局结束: {}胜", winner));
        }
    } else if let Some(checked) = snapshot.in_check_side {
        let checked_side = if checked > 0 { "红方" } else { "黑方" };
        status.push_str(&format!(" | 被将军: {}", checked_side));
    }

    status
}

pub fn format_session(session: &SessionDto) -> String {
    let mode = match session.play_mode {
        PlayMode::LocalPvp => "人人对战",
        PlayMode::HumanVsAi => "人机对战",
    };
    let difficulty = match session.ai_difficulty {
        AiDifficulty::Easy => "简单",
        AiDifficulty::Medium => "中等",
        AiDifficulty::Hard => "困难",
    };
    let human = if session.human_side > 0 {
        "玩家执黑"
    } else {
        "玩家执白"
    };
    format!("{mode} · {difficulty} · {human}")
}

pub fn human_input_enabled(snapshot: &SnapshotDto) -> bool {
    snapshot
        .session
        .as_ref()
        .map(|s| s.human_input_enabled)
        .unwrap_or(true)
}

pub fn piece_color_rgb(side: i8) -> (u8, u8, u8) {
    if side > 0 {
        (183, 34, 34)
    } else {
        (47, 42, 38)
    }
}
