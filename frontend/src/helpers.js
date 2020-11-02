import { createGlobalStyle } from "styled-components";

export const getRequestOptions = (method = "GET", body, contentType = "application/json") => ({
  headers: {
    "Content-Type": contentType,
    Authorization: "Bearer " + localStorage.getItem("access-token")
  },
  method,
  body
});

export const fetchData = async (path, body, method, contentType) => {
  const response = await fetch(path, getRequestOptions(method, body, contentType));
  let json = {};
  if (response.status === 401) {
    window.location.href = `${window.location.origin}/logout`;
  }
  if (response.status !== 204) {
    json = await response.json();
  }
  if (!response.ok) {
    return Promise.reject(json);
  }
  return Promise.resolve(json);
};

export const isAuthenticated = () => {
  const token = localStorage.getItem('access-token');
  return token !== null;
}

export const GlobalStyle = createGlobalStyle`
  body {
    margin: 0;
    background: #e5e5e5;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    color: #3a3a3a;
    font-size: 14px;
  }
`