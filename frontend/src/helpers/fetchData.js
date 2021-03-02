export const getRequestOptions = (method = "GET", body, contentType = "application/json") => ({
  headers: {
    "Content-Type": contentType,
    Authorization: "Bearer " + localStorage.getItem("access-token"),
  },
  method,
  body,
});

export const fetchData = async (path, body, method, contentType) => {
  return new Promise(async (resolve, reject) => {
    let response;
    try {
      response = await fetch(path, getRequestOptions(method, body, contentType));
    } catch (error) {
      return reject(error);
    }
    let json = {};
    try {
      json = await response.json();
    } catch (error) {
    }
    if (!response.ok || (json.hasOwnProperty("success") && !json.success)) {
      return reject(json);
    }
    return resolve(json);
  });
};