import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Header from './components/Header';
import Home from './components/Home';
import Features from './components/Features';
import Download from './components/Download';
import Community from './components/Community';
import Footer from './components/Footer';
import './App.css';

function App() {
  return (
    <Router>
      <div className="App">
        <Header />
        <Switch>
          <Route exact path="/" component={Home} />
          <Route path="/features" component={Features} />
          <Route path="/download" component={Download} />
          <Route path="/community" component={Community} />
        </Switch>
        <Footer />
      </div>
    </Router>
  );
}

export default App;
