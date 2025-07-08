use crate::thaiid::apdu::*;
use crate::thaiid::parser::decode_tis620;
use pcsc::{Context, Scope, ShareMode, Protocols};
use std::fs::File;
use std::io::Write;

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
    pub photo: Vec<u8>,
}

#[allow(dead_code)]
pub fn read_thai_id() -> ThaiIdInfo {
    let ctx = Context::establish(Scope::User).expect("PCSC init failed");

    let mut reader_buf = [0; 2048];
    let reader_list = ctx.list_readers(&mut reader_buf).expect("List readers failed");
    let mut readers = reader_list.clone();
    let reader_name = readers.next().expect("No reader found");

    let card = ctx.connect(reader_name, ShareMode::Shared, Protocols::ANY)
        .expect("Failed to connect to card");

    let mut response = [0; 256];

    let mut select_apdu = Vec::new();
    select_apdu.extend_from_slice(SELECT); // 0x00, 0xA4, 0x04, 0x00, 0x08
    select_apdu.extend_from_slice(THAI_CARD); // A0 00 00 00 54 48 00 01

    card.transmit(&select_apdu, &mut response).expect("SELECT THAI_CARD AID failed");

    let mut cid_buf = [0u8; 512];
    card.transmit(&CMD_CID, &mut cid_buf).unwrap();
    let cid = decode_tis620(&cid_buf);
    println!("CID: {}", cid);

    let mut th_name_buf = [0u8; 512];
    card.transmit(&CMD_THFULLNAME, &mut th_name_buf).unwrap();
    let th_name = decode_tis620(&th_name_buf);
    println!("TH Name: {}", th_name);

    let mut en_name_buf = [0u8; 512];
    card.transmit(&CMD_ENFULLNAME, &mut en_name_buf).unwrap();
    let en_name = decode_tis620(&en_name_buf);
    println!("EN Name: {}", en_name);

    let mut birth_buf = [0u8; 512];
    card.transmit(&CMD_BIRTH, &mut birth_buf).unwrap();
    let birth = decode_tis620(&birth_buf);
    println!("Birth: {}", birth);

    let mut gender_buf = [0u8; 512];
    card.transmit(&CMD_GENDER, &mut gender_buf).unwrap();
    let gender = decode_tis620(&gender_buf);
    println!("Gender: {}", gender);

    let mut issuer_buf = [0u8; 512];
    card.transmit(&CMD_ISSUER, &mut issuer_buf).unwrap();
    let issuer = decode_tis620(&issuer_buf);
    println!("Issuer: {}", issuer);

    let mut issue_date_buf = [0u8; 512];
    card.transmit(&CMD_ISSUE, &mut issue_date_buf).unwrap();
    let issue_date = decode_tis620(&issue_date_buf);
    println!("Issue Date: {}", issue_date);

    let mut expire_date_buf = [0u8; 512];
    card.transmit(&CMD_EXPIRE, &mut expire_date_buf).unwrap();
    let expire_date = decode_tis620(&expire_date_buf);
    println!("Expire Date: {}", expire_date);

    let mut address_buf = [0u8; 512];
    card.transmit(&CMD_ADDRESS, &mut address_buf).unwrap();
    let address = decode_tis620(&address_buf);
    println!("Address: {}", address);

    let mut photo: Vec<u8> = Vec::new();
    for cmd in CMD_PHOTOS.iter() {
        let mut buf = [0u8; 512];
        if let Ok(data_slice) = card.transmit(cmd, &mut buf) {
            photo.extend_from_slice(data_slice);
        }
    }

    let mut file = File::create("photo.jpg").unwrap();
    file.write_all(&photo).unwrap();

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
        photo,
    }
}
