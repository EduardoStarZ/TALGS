/*
 *
 * receiver.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

use ntex::web;
use ntex::util::BytesMut;
use crate::colors::color::Color;

pub async fn read_payload_to_string(mut payload : web::types::Payload) -> Option<String> {
    let mut bytes = BytesMut::new();
        while let Some(item) = ntex::util::stream_recv(&mut payload).await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let body : Option<String> = match String::from_utf8(bytes.into_iter().collect::<Vec<u8>>()) {
        Ok(value) => Some(value), 
        Err(err) => {
            println!("{}", err.to_string().warning());
            None
        } 
    };
    
    return body;
}
