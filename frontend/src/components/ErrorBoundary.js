import React from "react";
import { Error } from "../assets/Error";
import { withRouter } from "react-router-dom";

class ErrorBoundaryClass extends React.Component {
  constructor(props) {
    super(props);
    this.state = { error: null };
  }

  static getDerivedStateFromError(error) {
    return { error };
  }

  componentDidMount() {
    this.unlisten = this.props.history.listen((location, action) => {
      if (this.state.error) {
        this.setState({ error: null });
      }
    });
  }

  componentWillUnmount() {
    this.unlisten();
  }

  render() {
    if (this.state.error) {
      return this.props.fallback || <Error message={this.state.error} />;
    }

    return this.props.children;
  }
}

export const ErrorBoundary = withRouter(ErrorBoundaryClass);
