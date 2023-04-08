import logo from './logo.svg';
import './App.css';

// import { sprintf } from "sprintf-js";
// import packageJson from '../package.json';
import 'antd/dist/reset.css';
import { load } from "js/base.js";

function App() {
  load();

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
}

export default App;
