import React, { Component } from 'react';
import GraphiQL from 'graphiql';
import './App.css';
import 'graphiql/graphiql.css';

class App extends Component {
  graphQLFetcher(graphQLParams) {
    return fetch(window.location.origin + '/graphql', {
      method: 'post',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(graphQLParams),
    }).then(response => response.json());
  }

  render() {
    return (
      <GraphiQL fetcher={this.graphQLFetcher} />
    );
  }
}

export default App;
