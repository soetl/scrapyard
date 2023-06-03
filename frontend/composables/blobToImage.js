export async function blobToImage(blob, mime) {
  return `data:${mime};base64, ${arrayBufferToBase64(
    await blob.arrayBuffer()
  )}`;
}

function arrayBufferToBase64(buffer) {
  let binary = "";
  const bytes = new Uint8Array(buffer);
  const len = bytes.byteLength;
  for (let i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}
