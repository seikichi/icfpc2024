const TABLE =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

const RTABLE: { [key: string]: number } = {};
for (let i = 0; i < TABLE.length; i++) {
  RTABLE[TABLE[i]] = i;
}

export function decodeString(s: string): string | null {
  if (s[0] !== "S") {
    return null;
  }

  let result = "";
  for (let i = 1; i < s.length; i++) {
    result += TABLE[s.charCodeAt(i) - 33];
  }
  return result;
}

export function encodeString(s: string): string {
  let result = "S";
  for (let i = 0; i < s.length; i++) {
    result += String.fromCharCode(RTABLE[s[i]] + 33);
  }
  return result;
}
