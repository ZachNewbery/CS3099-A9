import styled from "styled-components";
import { colors } from "./colors";
import { fonts } from "./fonts";

export const StyledForm = styled.form`
  align-items: flex-start;
  display: flex;
  flex-flow: column nowrap;
  justify-content: center;
  align-items: center;
  width: 20rem;

  & > label {
    display: flex;
    flex-direction: column;
    font-size: 0.9rem;
    width: 100%;
    margin-bottom: 1.25rem;
    position: relative;

    & > input {
      margin: 0.3rem 0;
      padding: 0.45rem;
      border: 1px solid ${colors.lightGray};
      background: none !important;
      border-radius: 0.3rem;
      font: inherit;
      font-size: 1rem;
    }

    & > textarea {
      margin: 0.3rem 0;
      padding: 0.45rem;
      border: 1px solid ${colors.lightGray};
      background: none !important;
      border-radius: 0.3rem;
      font: inherit;
      font-size: 1rem;
    }

    & > .error {
      position: absolute;
      color: red;
      margin: 0.1rem 0.25rem;
      font-size: 0.8rem;
      text-align: center;
      top: 100%;
      left: 0;
      right: 0;
    }
  }

  & > button {
    cursor: pointer;
    outline: none;
    border: none;
    background: ${colors.blueGradient};
    box-shadow: ${colors.blueInsetShadow};
    padding: 0.5rem;
    width: 100%;
    color: ${colors.white};
    font: inherit;
    font-size: 1.15rem;
    font-family: ${fonts.accent};
    letter-spacing: 1.5px;
    height: 40px;
    border-radius: 20px;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: all 0.3s;
    &:hover {
      color: white;
      background: ${colors.lightBlueGradient};
    }
  }
`;
