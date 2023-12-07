

export const USER_KEY = "login_user_id";



/**
 * 生成uuid
 */
export function uuid() {
  let uuidValue = "", k, randomValue;
  for (k = 0; k < 32; k++) {
    randomValue = Math.random() * 16 | 0;

    if (k == 8 || k == 12 || k == 16 || k == 20) {
      uuidValue += "-"
    }
    uuidValue += (k == 12 ? 4 : (k == 16 ? (randomValue & 3 | 8) : randomValue)).toString(16);
  }
  return uuidValue;
}



/**
 * base64转File 函数
 * @param base64
 * @param fileName
 */
export function base64ToFile(base64: string, fileName: string): File {
  let name = fileName.split('.')[0];
  let arr = base64.split(',');
  let fileExt = fileName.substring(fileName.lastIndexOf('.'), fileName.length);
  let type = arr[0].match(/:(.*?);/)![1];
  // let fileExt = type.split('/')[1];
  let bstr = atob(arr[1]);
  let n = bstr.length;
  let u8arr = new Uint8Array(n);
  while (n--) {
    u8arr[n] = bstr.charCodeAt(n);
  }
  return new File([u8arr], `${name}` + fileExt, {
    type: type,
  });
}