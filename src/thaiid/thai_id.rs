use crate::thaiid::apdu::*;
use crate::thaiid::parser::decode_tis620;
use pcsc::{Context, Scope, ShareMode, Protocols};
use base64::{engine::general_purpose, Engine as _};

#[allow(dead_code)]
pub struct ThaiIdInfo {
    pub cid: String,
    pub th_name: String,
    pub en_name: String,
    pub birth: String,
    pub gender: String,
    pub issuer: String,
    pub issue_date: String,
    pub expire_date: String,
    pub address: String,
    pub photo_base64: String,
}

#[allow(dead_code)]
pub fn read_thai_id() -> ThaiIdInfo {
    let ctx = Context::establish(Scope::User).expect("PCSC init failed");
    let mut reader_buf = [0; 2048];
    let reader_list = ctx.list_readers(&mut reader_buf).expect("List readers failed");
    let mut readers = reader_list.clone();
    let reader_name = readers.next().expect("No reader found");
    let card = ctx.connect(reader_name, ShareMode::Shared, Protocols::ANY).expect("Card connect failed");
    let get_response_prefix: &[u8] = &[0x00, 0xC0, 0x00, 0x00];
    let mut rapdu_buf = [0; 512];

    let mut select_apdu = Vec::new();
    select_apdu.extend_from_slice(SELECT);
    select_apdu.extend_from_slice(THAI_CARD);
    card.transmit(&select_apdu, &mut rapdu_buf).expect("SELECT failed");

    macro_rules! read_field {
        ($cmd:expr, $desc:expr) => {{
            card.transmit($cmd, &mut rapdu_buf).expect(concat!("Send ", $desc, " failed"));
            let mut apdu = get_response_prefix.to_vec();
            apdu.push($cmd[$cmd.len() - 1]);
            let data = card.transmit(&apdu, &mut rapdu_buf).expect(concat!("Recv ", $desc, " failed"));
            if data.len() < 2 || data[data.len() - 2] != 0x90 || data[data.len() - 1] != 0x00 {
                panic!("{} response error", $desc);
            }
            decode_tis620(&data[..data.len() - 2])
        }};
    }

    let cid = read_field!(&CMD_CID, "CID");
    let th_name = read_field!(&CMD_THFULLNAME, "TH Name");
    let en_name = read_field!(&CMD_ENFULLNAME, "EN Name");
    let birth = read_field!(&CMD_BIRTH, "Birth");
    let gender = read_field!(&CMD_GENDER, "Gender");
    let issuer = read_field!(&CMD_ISSUER, "Issuer");
    let issue_date = read_field!(&CMD_ISSUE, "Issue Date");
    let expire_date = read_field!(&CMD_EXPIRE, "Expire Date");
    let address = read_field!(&CMD_ADDRESS, "Address");

    let mut photo: Vec<u8> = Vec::new();
    for cmd in CMD_PHOTOS.iter() {
        card.transmit(cmd, &mut rapdu_buf).expect("Photo command failed");
        let mut apdu = get_response_prefix.to_vec();
        apdu.push(cmd[cmd.len() - 1]);
        let part = card.transmit(&apdu, &mut rapdu_buf).expect("GET RESPONSE for photo failed");
        if part.len() < 2 || part[part.len() - 2] != 0x90 || part[part.len() - 1] != 0x00 {
            eprintln!("Photo part warning");
        }
        photo.extend_from_slice(&part[..part.len() - 2]);
    }

    let photo_base64 = general_purpose::STANDARD.encode(&photo);

    ThaiIdInfo {
        cid,
        th_name,
        en_name,
        birth,
        gender,
        issuer,
        issue_date,
        expire_date,
        address,
        photo_base64,
    }
}
