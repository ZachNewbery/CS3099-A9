import { createGlobalStyle } from "styled-components";

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