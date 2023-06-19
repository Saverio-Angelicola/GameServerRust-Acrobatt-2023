use geo::{point, HaversineDistance};

use crate::{
    core::game::{game_config::Flag, items::{Item, ItemInLoading}},
    server::server_config::Client,
};

use super::{player_init::Position, player_response::{CheckCoordsPlayerResponse, CheckPointCoordsResponse, CheckTrapCoordsResponse}};

pub fn check_flag_coord(flag: &Flag, current_position: Position) -> CheckPointCoordsResponse {
    let mut is_visible = false;
    let mut is_capturable = false;
    if distance(
        current_position.x,
        current_position.y,
        flag.position.x,
        flag.position.y,
    ) <= flag.visibility_radius
    {
        is_visible = true;
    }

    let distance = distance(
        current_position.x,
        current_position.y,
        flag.position.x,
        flag.position.y,
    );

    if distance <= flag.action_radius {
        is_capturable = true;
    }

    return CheckPointCoordsResponse {
        point_id: flag.id.to_owned(),
        visible: is_visible,
        capturable: is_capturable,
    };
}

pub fn check_item_coord(item: &Item, current_position: Position) -> CheckPointCoordsResponse {
    let mut is_visible = false;
    let mut is_capturable = false;
    if distance(
        current_position.x,
        current_position.y,
        item.position.x,
        item.position.y,
    ) <= item.visibility_radius
    {
        is_visible = true;
    }

    let distance = distance(
        current_position.x,
        current_position.y,
        item.position.x,
        item.position.y,
    );

    if distance <= item.action_radius {
        is_capturable = true
    }

    return CheckPointCoordsResponse {
        point_id: item.id.to_owned(),
        visible: is_visible,
        capturable: is_capturable,
    };
}

pub fn check_trap_coord(item: &ItemInLoading, current_position: Position) -> CheckTrapCoordsResponse {
    let mut is_visible = false;
    let mut is_capturable = false;
    if distance(
        current_position.x,
        current_position.y,
        item.item.position.x,
        item.item.position.y,
    ) <= item.item.visibility_radius
    {
        is_visible = true;
    }

    let distance = distance(
        current_position.x,
        current_position.y,
        item.item.position.x,
        item.item.position.y,
    );

    if distance <= item.item.action_radius {
        is_capturable = true
    }

    return CheckTrapCoordsResponse {
        point_id: item.item.id.to_owned(),
        visible: is_visible,
        capturable: is_capturable,
        team_id: item.team_id,
    };
}

pub fn check_client_coord(
    client: &Client,
    current_position: Position,
) -> CheckCoordsPlayerResponse {
    let mut is_visible = false;
    if distance(
        current_position.x,
        current_position.y,
        client.position.x,
        client.position.y,
    ) <= 10
    {
        is_visible = true;
    }

    return CheckCoordsPlayerResponse {
        client_id: client.id.to_owned(),
        is_visible,
    };
}

fn distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> u64 {
    let coord1 = point!(x: lat1, y: lon1);

    let coord2 = point!(x: lat2, y: lon2);

    return coord1.haversine_distance(&coord2).round() as u64;
}
