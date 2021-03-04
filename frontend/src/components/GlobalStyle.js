import { createGlobalStyle } from "styled-components";

export const GlobalStyle = createGlobalStyle`
  html {
    height: 100%;
  }
  body {
    margin: 0;
    font-family: "Nunito", -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    color: #3a3a3a;
    background-color: #f8f9f9;
    font-size: 14px;
    height: 100%;
  }
  iframe {
    display: none;
  }
  * {
    box-sizing: border-box;
  }
  #root {
    display: flex;
    flex-flow: column nowrap;
    height: 100%;
  }

  main {
    display: flex;
    justify-content: center;
    margin: 0 auto;
    width: 50rem;
    max-width: 95%;
    padding-bottom: 5rem;
  }
`;
