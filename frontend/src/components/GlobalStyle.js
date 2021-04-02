import { createGlobalStyle } from "styled-components";

export const GlobalStyle = createGlobalStyle`
  html {
    height: 100%;
    &::-webkit-scrollbar {
      width: 6px;
      height: 6px;
    }

    &::-webkit-scrollbar-corner {
      background: #f8f9f9;
    }

    &::-webkit-scrollbar-thumb {
      border-radius: 3px;
      background: #bdbdbd;
      transition: background 0.3s;
      &:hover {
        background: #a2a2a2;
      }
    }
  }
  body {
    margin: 0;
    font-family: "Nunito", -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    color: #3a3a3a;
    background-color: #f8f9f9;
    font-size: 14px;
    height: 100%;
    overflow: auto scroll;
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
    height: calc(100vh - 5rem + 1px);
  }
`;
