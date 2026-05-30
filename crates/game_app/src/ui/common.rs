use game_view::SnapshotDto;

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

pub fn piece_color_rgb(side: i8) -> (u8, u8, u8) {
    if side > 0 {
        (183, 34, 34)
    } else {
        (47, 42, 38)
    }
}
