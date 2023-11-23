pub mod http {
    use std::env;
    use base64::Engine;
    use protobuf::rustproto::exts::generate_accessors;
    use rsa::Error::Pkcs8;
    use rsa::Pkcs1v15Encrypt;
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
    use entity::ProstMessage;

    #[tauri::command]
    pub async fn login() {
        let client = reqwest::Client::new();
    }

    #[tauri::command]
    pub fn rsa_gen() {
        use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
        use rand::thread_rng;
        use rsa::pkcs1::LineEnding;
        use base64::{engine::general_purpose};

        let mut rng = rand::thread_rng();
        let bits = 1024;

        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let result = public_key.to_public_key_pem(LineEnding::CRLF).unwrap();
        let res2 = private_key.to_pkcs8_pem(LineEnding::CRLF).unwrap();

        println!("result1 : {:?}", result);
        println!("result2 : {:?}", res2.as_str());
        println!("private key : {:?}", private_key);
        println!("public key : {:?}", public_key);

        let raw_str = b"admin";
        let encode_str = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str).unwrap();

        let base64_encode = general_purpose::STANDARD_NO_PAD.encode(encode_str.clone());
        println!("{}", base64_encode);


        let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &encode_str).unwrap();


        println!("{:?}", &decode_str[..]);
        println!("{:?}", &raw_str[..]);

        let str = String::from_utf8(decode_str.clone());

        println!("{:?}", str);

        let vec = general_purpose::STANDARD_NO_PAD.decode(base64_encode).unwrap();
        assert_eq!(vec, encode_str);

        let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &vec).unwrap();
        let str = String::from_utf8(decode_str.clone());
        println!("{:?}", str);

        println!("");

        encodeStr();
    }


    pub fn encodeStr() {
        use base64::engine::general_purpose;
        use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
        use rsa::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};

        let server_public_key = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC3ktCxwURY+Pkz49sDbmy2/WWv
j6X3noeoh0coEY41DO5meYIAebkIqiYR2Hkhkf6s0SIdZT1gmZQQx2ZPmb/bI4L2
CE0ILa/ZabzIHgcBPdouzuj/whV/WhKx0y5uACsaEg+Khr8rmBbh5EGyw4EUWnA1
4/pUds5rdAwpfZiM9wIDAQAB
-----END PUBLIC KEY-----";
        let server_public_key = RsaPublicKey::from_public_key_pem(&server_public_key).unwrap();
        let raw_str = b"admin";
        let mut rng = rand::thread_rng();
        let encode_str = server_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str).unwrap();
        let base64_encode = general_purpose::STANDARD_NO_PAD.encode(encode_str.clone());

        println!("{}", base64_encode);
    }
}