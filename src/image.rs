use std::io::{self, Write};

use base64::{Engine, engine::general_purpose::STANDARD};

const IMAGE_BEGIN: &[u8] = b"\x1b_G";
const IMAGE_END: &[u8] = b"\x1b\\";
const IMAGE_SEPARATE: u8 = b';';
const IMAGE_CONTROL_SPLIT: &str = ",";

fn serialize_gr_command(cmd: &Vec<(char, &str)>, payload: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(IMAGE_BEGIN);

    let cmd_str = cmd
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join(IMAGE_CONTROL_SPLIT);

    result.extend_from_slice(cmd_str.as_bytes());

    result.push(IMAGE_SEPARATE);
    result.extend_from_slice(payload);

    result.extend_from_slice(IMAGE_END);
    result
}

fn write_chunked(cmd: &mut Vec<(char, &str)>, data: &[u8]) -> io::Result<()> {
    let encoded_data = STANDARD.encode(data);

    let count = encoded_data.as_bytes().chunks(4096).count();
    encoded_data
        .as_bytes()
        .chunks(4096)
        .enumerate()
        .for_each(|(i, chunk)| {
            let m = if i == count - 1 { "0" } else { "1" };
            cmd.push(('m', m));

            io::stdout()
                .write_all(&serialize_gr_command(cmd, chunk))
                .unwrap();
            io::stdout().flush().unwrap();

            cmd.pop();
        });

    Ok(())
}

pub fn display_img(data: &[u8]) -> io::Result<()> {
    let mut cmd = vec![('a', "T"), ('f', "100"), ('C', "1")];
    write_chunked(&mut cmd, data)
}
