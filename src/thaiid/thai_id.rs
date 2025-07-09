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

    let get_response_prefix: &[u8] = &[0x00, 0xC0, 0x00, 0x00];

    let mut rapdu_buf = [0; 512]; // General purpose buffer, adjust size if needed

    let mut select_apdu = Vec::new();
    select_apdu.extend_from_slice(SELECT);
    select_apdu.extend_from_slice(THAI_CARD);

    let _select_response = card.transmit(&select_apdu, &mut rapdu_buf)
        .expect("SELECT THAI_CARD AID failed");

    // --- CID ---
    let _initial_cid_response = card.transmit(&CMD_CID, &mut rapdu_buf)
        .expect("CMD_CID failed");

    let le_cid = CMD_CID[CMD_CID.len() - 1];
    let mut get_data_apdu_cid = Vec::new();
    get_data_apdu_cid.extend_from_slice(get_response_prefix);
    get_data_apdu_cid.push(le_cid);

    let cid_response_slice = card.transmit(&get_data_apdu_cid, &mut rapdu_buf)
        .expect("GET RESPONSE for CID failed");

    if cid_response_slice.len() < 2 || cid_response_slice[cid_response_slice.len() - 2] != 0x90 || cid_response_slice[cid_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for CID failed with status: {:02X}{:02X}", cid_response_slice[cid_response_slice.len() - 2], cid_response_slice[cid_response_slice.len() - 1]);
    }
    let cid_bytes = cid_response_slice[..cid_response_slice.len() - 2].to_vec();
    let cid = decode_tis620(&cid_bytes);
    println!("CID: {}", cid);

    // --- TH Fullname ---
    let _initial_thname_response = card.transmit(&CMD_THFULLNAME, &mut rapdu_buf)
        .expect("CMD_THFULLNAME failed");
    let le_thname = CMD_THFULLNAME[CMD_THFULLNAME.len() - 1];
    let mut get_data_apdu_thname = Vec::new();
    get_data_apdu_thname.extend_from_slice(get_response_prefix);
    get_data_apdu_thname.push(le_thname);
    let th_name_response_slice = card.transmit(&get_data_apdu_thname, &mut rapdu_buf)
        .expect("GET RESPONSE for TH Name failed");
    if th_name_response_slice.len() < 2 || th_name_response_slice[th_name_response_slice.len() - 2] != 0x90 || th_name_response_slice[th_name_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for TH Name failed with status: {:02X}{:02X}", th_name_response_slice[th_name_response_slice.len() - 2], th_name_response_slice[th_name_response_slice.len() - 1]);
    }
    let th_name_bytes = th_name_response_slice[..th_name_response_slice.len() - 2].to_vec();
    let th_name = decode_tis620(&th_name_bytes);
    println!("TH Name: {}", th_name);

    // --- EN Fullname ---
    let _initial_enname_response = card.transmit(&CMD_ENFULLNAME, &mut rapdu_buf)
        .expect("CMD_ENFULLNAME failed");
    let le_enname = CMD_ENFULLNAME[CMD_ENFULLNAME.len() - 1];
    let mut get_data_apdu_enname = Vec::new();
    get_data_apdu_enname.extend_from_slice(get_response_prefix);
    get_data_apdu_enname.push(le_enname);
    let en_name_response_slice = card.transmit(&get_data_apdu_enname, &mut rapdu_buf)
        .expect("GET RESPONSE for EN Name failed");
    if en_name_response_slice.len() < 2 || en_name_response_slice[en_name_response_slice.len() - 2] != 0x90 || en_name_response_slice[en_name_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for EN Name failed with status: {:02X}{:02X}", en_name_response_slice[en_name_response_slice.len() - 2], en_name_response_slice[en_name_response_slice.len() - 1]);
    }
    let en_name_bytes = en_name_response_slice[..en_name_response_slice.len() - 2].to_vec();
    let en_name = decode_tis620(&en_name_bytes);
    println!("EN Name: {}", en_name);

    // --- Birth ---
    let _initial_birth_response = card.transmit(&CMD_BIRTH, &mut rapdu_buf)
        .expect("CMD_BIRTH failed");
    let le_birth = CMD_BIRTH[CMD_BIRTH.len() - 1];
    let mut get_data_apdu_birth = Vec::new();
    get_data_apdu_birth.extend_from_slice(get_response_prefix);
    get_data_apdu_birth.push(le_birth);
    let birth_response_slice = card.transmit(&get_data_apdu_birth, &mut rapdu_buf)
        .expect("GET RESPONSE for Birth failed");
    if birth_response_slice.len() < 2 || birth_response_slice[birth_response_slice.len() - 2] != 0x90 || birth_response_slice[birth_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for Birth failed with status: {:02X}{:02X}", birth_response_slice[birth_response_slice.len() - 2], birth_response_slice[birth_response_slice.len() - 1]);
    }
    let birth_bytes = birth_response_slice[..birth_response_slice.len() - 2].to_vec();
    let birth = decode_tis620(&birth_bytes);
    println!("Birth: {}", birth);

    // --- Gender ---
    let _initial_gender_response = card.transmit(&CMD_GENDER, &mut rapdu_buf)
        .expect("CMD_GENDER failed");
    let le_gender = CMD_GENDER[CMD_GENDER.len() - 1];
    let mut get_data_apdu_gender = Vec::new();
    get_data_apdu_gender.extend_from_slice(get_response_prefix);
    get_data_apdu_gender.push(le_gender);
    let gender_response_slice = card.transmit(&get_data_apdu_gender, &mut rapdu_buf)
        .expect("GET RESPONSE for Gender failed");
    if gender_response_slice.len() < 2 || gender_response_slice[gender_response_slice.len() - 2] != 0x90 || gender_response_slice[gender_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for Gender failed with status: {:02X}{:02X}", gender_response_slice[gender_response_slice.len() - 2], gender_response_slice[gender_response_slice.len() - 1]);
    }
    let gender_bytes = gender_response_slice[..gender_response_slice.len() - 2].to_vec();
    let gender = decode_tis620(&gender_bytes);
    println!("Gender: {}", gender);

    // --- Issuer ---
    let _initial_issuer_response = card.transmit(&CMD_ISSUER, &mut rapdu_buf)
        .expect("CMD_ISSUER failed");
    let le_issuer = CMD_ISSUER[CMD_ISSUER.len() - 1];
    let mut get_data_apdu_issuer = Vec::new();
    get_data_apdu_issuer.extend_from_slice(get_response_prefix);
    get_data_apdu_issuer.push(le_issuer);
    let issuer_response_slice = card.transmit(&get_data_apdu_issuer, &mut rapdu_buf)
        .expect("GET RESPONSE for Issuer failed");
    if issuer_response_slice.len() < 2 || issuer_response_slice[issuer_response_slice.len() - 2] != 0x90 || issuer_response_slice[issuer_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for Issuer failed with status: {:02X}{:02X}", issuer_response_slice[issuer_response_slice.len() - 2], issuer_response_slice[issuer_response_slice.len() - 1]);
    }
    let issuer_bytes = issuer_response_slice[..issuer_response_slice.len() - 2].to_vec();
    let issuer = decode_tis620(&issuer_bytes);
    println!("Issuer: {}", issuer);

    // --- Issue Date ---
    let _initial_issue_response = card.transmit(&CMD_ISSUE, &mut rapdu_buf)
        .expect("CMD_ISSUE failed");
    let le_issue = CMD_ISSUE[CMD_ISSUE.len() - 1];
    let mut get_data_apdu_issue = Vec::new();
    get_data_apdu_issue.extend_from_slice(get_response_prefix);
    get_data_apdu_issue.push(le_issue);
    let issue_date_response_slice = card.transmit(&get_data_apdu_issue, &mut rapdu_buf)
        .expect("GET RESPONSE for Issue Date failed");
    if issue_date_response_slice.len() < 2 || issue_date_response_slice[issue_date_response_slice.len() - 2] != 0x90 || issue_date_response_slice[issue_date_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for Issue Date failed with status: {:02X}{:02X}", issue_date_response_slice[issue_date_response_slice.len() - 2], issue_date_response_slice[issue_date_response_slice.len() - 1]);
    }
    let issue_date_bytes = issue_date_response_slice[..issue_date_response_slice.len() - 2].to_vec();
    let issue_date = decode_tis620(&issue_date_bytes);
    println!("Issue Date: {}", issue_date);

    // --- Expire Date ---
    let _initial_expire_response = card.transmit(&CMD_EXPIRE, &mut rapdu_buf)
        .expect("CMD_EXPIRE failed");
    let le_expire = CMD_EXPIRE[CMD_EXPIRE.len() - 1];
    let mut get_data_apdu_expire = Vec::new();
    get_data_apdu_expire.extend_from_slice(get_response_prefix);
    get_data_apdu_expire.push(le_expire);
    let expire_date_response_slice = card.transmit(&get_data_apdu_expire, &mut rapdu_buf)
        .expect("GET RESPONSE for Expire Date failed");
    if expire_date_response_slice.len() < 2 || expire_date_response_slice[expire_date_response_slice.len() - 2] != 0x90 || expire_date_response_slice[expire_date_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for Expire Date failed with status: {:02X}{:02X}", expire_date_response_slice[expire_date_response_slice.len() - 2], expire_date_response_slice[expire_date_response_slice.len() - 1]);
    }
    let expire_date_bytes = expire_date_response_slice[..expire_date_response_slice.len() - 2].to_vec();
    let expire_date = decode_tis620(&expire_date_bytes);
    println!("Expire Date: {}", expire_date);

    // --- Address ---
    let _initial_address_response = card.transmit(&CMD_ADDRESS, &mut rapdu_buf)
        .expect("CMD_ADDRESS failed");
    let le_address = CMD_ADDRESS[CMD_ADDRESS.len() - 1];
    let mut get_data_apdu_address = Vec::new();
    get_data_apdu_address.extend_from_slice(get_response_prefix);
    get_data_apdu_address.push(le_address);
    let address_response_slice = card.transmit(&get_data_apdu_address, &mut rapdu_buf)
        .expect("GET RESPONSE for Address failed");
    if address_response_slice.len() < 2 || address_response_slice[address_response_slice.len() - 2] != 0x90 || address_response_slice[address_response_slice.len() - 1] != 0x00 {
        panic!("GET RESPONSE for Address failed with status: {:02X}{:02X}", address_response_slice[address_response_slice.len() - 2], address_response_slice[address_response_slice.len() - 1]);
    }
    let address_bytes = address_response_slice[..address_response_slice.len() - 2].to_vec();
    let address = decode_tis620(&address_bytes);
    println!("Address: {}", address);

    // --- Photo ---
    let mut photo: Vec<u8> = Vec::new();
    for cmd in CMD_PHOTOS.iter() {
        let _initial_photo_response = card.transmit(cmd, &mut rapdu_buf)
            .expect("Photo command failed");
        let le_photo = cmd[cmd.len() - 1];
        let mut get_data_apdu_photo = Vec::new();
        get_data_apdu_photo.extend_from_slice(get_response_prefix);
        get_data_apdu_photo.push(le_photo);
        let photo_response_slice = card.transmit(&get_data_apdu_photo, &mut rapdu_buf)
            .expect("GET RESPONSE for Photo part failed");
        if photo_response_slice.len() < 2 || photo_response_slice[photo_response_slice.len() - 2] != 0x90 || photo_response_slice[photo_response_slice.len() - 1] != 0x00 {
            eprintln!("Warning: GET RESPONSE for Photo part failed with status: {:02X}{:02X}", photo_response_slice[photo_response_slice.len() - 2], photo_response_slice[photo_response_slice.len() - 1]);
        }
        let photo_part = photo_response_slice[..photo_response_slice.len() - 2].to_vec();
        photo.extend_from_slice(&photo_part);
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
