function getPoem() {
  return new Promise((resolve, reject) => {
    jinrishici.load(function (result) {
      resolve(result);
    });
  })
}
