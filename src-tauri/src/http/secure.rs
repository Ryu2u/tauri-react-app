use base64::Engine;
use std::fs::File;
use log::info;
use std::io::Read;

/// 获取公钥
pub fn get_public_key() -> String {
    use rsa::{RsaPublicKey};
    use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
    use base64::{engine::general_purpose};

    let mut file: File = std::fs::File::open("public_key.txt").unwrap();
    let mut public_key = String::new();
    file.read_to_string(&mut public_key).expect("can't read file");
    let local_public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();
    let doc = local_public_key.to_public_key_der().unwrap();
    let base64_encode = general_purpose::STANDARD_NO_PAD.encode(doc.to_vec());
    base64_encode
}


/// 使用后端公钥加密需要发送给后端的内容
/// 返回的String 为base64 加密后的字符串
pub fn encode_msg(raw_str: &[u8]) -> String {
    use base64::engine::general_purpose;
    use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
    use rsa::pkcs8::{DecodePublicKey};
    let server_public_key = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQC3ktCxwURY+Pkz49sDbmy2/WWv
j6X3noeoh0coEY41DO5meYIAebkIqiYR2Hkhkf6s0SIdZT1gmZQQx2ZPmb/bI4L2
CE0ILa/ZabzIHgcBPdouzuj/whV/WhKx0y5uACsaEg+Khr8rmBbh5EGyw4EUWnA1
4/pUds5rdAwpfZiM9wIDAQAB
-----END PUBLIC KEY-----";
    let server_public_key = RsaPublicKey::from_public_key_pem(&server_public_key).unwrap();
    let mut rng = rand::thread_rng();
    let encode_str = server_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str)
        .unwrap();
    let base64_encode = general_purpose::STANDARD_NO_PAD.encode(&encode_str[..]);
    info!("{}", base64_encode);
    base64_encode
}

/// 使用本地私钥解密后端加密的内容
/// msg 为base64 加密后的字符串
pub fn decode_msg(msg: &str) -> String {
    use base64::engine::general_purpose;
    use rsa::{Pkcs1v15Encrypt};
    use rsa::pkcs8::{DecodePrivateKey};
    use rsa::{RsaPrivateKey};

    let mut file: File = std::fs::File::open("private_key.txt").unwrap();
    let mut private_key = String::new();
    file.read_to_string(&mut private_key).expect("can't read file");
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key).unwrap();

    let vec = general_purpose::STANDARD.decode(msg.to_string()).unwrap();

    let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &vec).unwrap();
    let res = String::from_utf8(decode_str);
    info!("{:?}", res);
    match res {
        Ok(str) => {
            str
        }
        _ => {
            "".to_string()
        }
    }
}


#[cfg(test)]
mod test {
    use crate::http::secure::{decode_msg, encode_msg, get_public_key};

    #[test]
    fn tes_rsa_key() {
        use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
        use rsa::pkcs8::{DecodePublicKey, EncodePublicKey, EncodePrivateKey};
        use rand::thread_rng;
        use std::path::Path;
        use rsa::pkcs1::LineEnding;
        use base64::{Engine, engine::general_purpose};

        let mut rng = rand::thread_rng();
        let bits = 1024;

        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let result = public_key.to_public_key_pem(LineEnding::CRLF).unwrap();
        let res2 = private_key.to_pkcs8_pem(LineEnding::CRLF).unwrap();

        /// 将生成的公钥写入指定的文件
        // public_key.write_public_key_pem_file(Path::new("C:/Users/Administrator/Desktop/public_key.txt"), LineEnding::CRLF).unwrap();
        /// 将生成的秘钥写入指定的文件
        // private_key.write_pkcs8_pem_file(Path::new("C:/Users/Administrator/Desktop/private_key.txt"), LineEnding::CRLF).unwrap();

        info!("result1 : {:?}", result);
        info!("result2 : {:?}", res2.as_str());
        info!("private key : {:?}", private_key);
        info!("public key : {:?}", public_key);

        let raw_str = b"admin";

        let encode_str = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str).unwrap();

        let base64_encode = general_purpose::STANDARD.encode(encode_str.clone());

        info!("{:?}", base64_encode);

        let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &encode_str).unwrap();


        info!("{:?}", &decode_str[..]);
        info!("{:?}", &raw_str[..]);

        let str = String::from_utf8(decode_str.clone());

        info!("{:?}", str);

        let vec = general_purpose::STANDARD.decode(base64_encode).unwrap();
        assert_eq!(vec, encode_str);

        let decode_str = private_key.decrypt(Pkcs1v15Encrypt, &vec).unwrap();
        let str = String::from_utf8(decode_str.clone());
        info!("{:?}", str);
    }

    #[test]
    fn encode_and_decode() {
        use base64::{Engine, engine::general_purpose};
        use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
        use rand::thread_rng;
        use rsa::pkcs1::LineEnding;
        use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
        use std::fs::File;
        use std::io::Read;

        let mut file: File = File::open("public_key.txt").unwrap();
        let mut public_key = String::new();
        file.read_to_string(&mut public_key).expect("can't read file");
        let local_public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();

        let mut rng = rand::thread_rng();
        let raw_string = "admin".to_string();
        let raw_str = b"admin";
        let encode_str = local_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, raw_str).unwrap();
        let base64_encode = general_purpose::STANDARD.encode(encode_str);
        info!("{:?}", base64_encode);
        let decode_str = decode_msg(&base64_encode);
        assert_eq!(raw_string, decode_str);
    }
}


