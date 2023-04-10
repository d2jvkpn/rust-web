// import logo from './logo.svg';
import './App.css';
import 'antd/dist/reset.css';

import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import LoginPage from "pages/login_page.jsx";
import HomePage from "pages/home_page.jsx";

import { sprintf } from "sprintf-js";
import packageJson from '../package.json';
import { load, getPublicUrl } from "js/base.js";
import { loadLang, getSet } from "locales/index.js";

function App() {
  init();

  /*
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
  */

  let basename = getPublicUrl();

  // return (<Router basename={basename}>
  return (<Router>
    <Routes>
      <Route exact path={basename + "/login"} element={<LoginPage/>}/>
      <Route exact path={basename + "/home"} element={<HomePage/>}/>
      <Route path="*" element={<LoginPage/>}/>
    </Routes>
  </Router>);
}

function init() {
  load();

  window.UILanguage = loadLang(localStorage.getItem("Language") || navigator.language);
  let langCommon = getSet("common"); // let langCommon = window.UILanguage.common;
  let welcome = sprintf(langCommon["welcome"], "d2jvkpn");
  console.log(`~~~ ${welcome}, rust-web-frontend version: ${packageJson.version}`);
}

export default App;
